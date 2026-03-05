//! PHI node elimination pass.
//!
//! Transforms SSA PHI nodes into explicit load/store operations through
//! stack slots, making the IR suitable for instruction selection.

use super::cfg::Cfg;
use super::FunctionPass;
use crate::ir::function::{BasicBlock, BlockLabel, Function};
use crate::ir::stmt::{PhiStmt, Stmt, StmtInner};
use crate::ir::types::Dtype;
use crate::ir::value::{LocalVariable, Operand};
use std::collections::HashMap;

pub struct PhiEliminationPass;

impl FunctionPass for PhiEliminationPass {
    fn run(&self, func: &mut Function) {
        let Some(blocks) = func.blocks.take() else {
            return;
        };

        if !has_phis(&blocks) {
            func.blocks = Some(blocks);
            return;
        }

        let (new_blocks, next_vreg) = PhiEliminator::new(&blocks, func.next_vreg).run();
        func.blocks = Some(new_blocks);
        func.next_vreg = next_vreg;
    }
}

fn has_phis(blocks: &[BasicBlock]) -> bool {
    blocks
        .iter()
        .flat_map(|block| block.stmts.iter())
        .any(|s| matches!(s.inner, StmtInner::Phi(_)))
}

struct PhiEliminator<'a> {
    blocks: &'a [BasicBlock],
    next_vreg: usize,
}

impl<'a> PhiEliminator<'a> {
    fn new(blocks: &'a [BasicBlock], next_vreg: usize) -> Self {
        Self { blocks, next_vreg }
    }

    fn run(mut self) -> (Vec<BasicBlock>, usize) {
        let mut parsed = self.parse_blocks();
        let cfg = Cfg::from_blocks(self.blocks);

        let mut slots = SlotAllocator::new();
        let phi_loads = self.build_phi_loads(&parsed, &mut slots);

        let mut edges = EdgeSplitter::new(next_basic_block_id(cfg.labels()));
        self.place_phi_stores(&parsed, &cfg, &slots, &mut edges);
        edges.patch_terminators(&mut parsed, cfg.labels());

        let next_vreg = self.next_vreg;
        let blocks = self.assemble(parsed, cfg.labels().to_vec(), slots, phi_loads, edges);
        (blocks, next_vreg)
    }

    fn parse_blocks(&self) -> Vec<ParsedBlock> {
        self.blocks.iter().map(ParsedBlock::from_block).collect()
    }

    fn build_phi_loads(
        &mut self,
        parsed: &[ParsedBlock],
        slots: &mut SlotAllocator,
    ) -> Vec<Vec<Stmt>> {
        parsed
            .iter()
            .map(|block| {
                block
                    .phis
                    .iter()
                    .map(|phi| {
                        let slot = slots.get_or_alloc(&phi.dst, &mut self.next_vreg);
                        Stmt::as_load(phi.dst.clone(), slot)
                    })
                    .collect()
            })
            .collect()
    }

    fn place_phi_stores(
        &mut self,
        parsed: &[ParsedBlock],
        cfg: &Cfg,
        slots: &SlotAllocator,
        edges: &mut EdgeSplitter,
    ) {
        for (block_idx, block) in parsed.iter().enumerate() {
            if block.phis.is_empty() {
                continue;
            }

            for &pred_idx in cfg.predecessors(block_idx) {
                let stores = slots.build_stores(&block.phis, cfg.label(pred_idx));
                if stores.is_empty() {
                    continue;
                }

                if cfg.successors(pred_idx).len() == 1 {
                    edges.insert_at_pred(pred_idx, stores);
                } else {
                    edges.split(pred_idx, block_idx, stores);
                }
            }
        }
    }

    fn assemble(
        self,
        parsed: Vec<ParsedBlock>,
        labels: Vec<BlockLabel>,
        slots: SlotAllocator,
        phi_loads: Vec<Vec<Stmt>>,
        edges: EdgeSplitter,
    ) -> Vec<BasicBlock> {
        let entry_idx = labels
            .iter()
            .position(|l| matches!(l, BlockLabel::Function(_)))
            .unwrap_or(0);

        let allocas = slots.into_allocas();
        let split_count = edges.split_count();
        let mut result = Vec::with_capacity(parsed.len() + split_count);

        for (idx, block) in parsed.into_iter().enumerate() {
            let mut stmts = Vec::new();

            if idx == entry_idx {
                stmts.extend(allocas.iter().cloned());
            }

            stmts.extend(phi_loads[idx].iter().cloned());

            if let Some(inserts) = edges.pending_inserts.get(&idx) {
                insert_before_terminator(&mut stmts, block.body, inserts.clone());
            } else {
                stmts.extend(block.body);
            }

            result.push(BasicBlock {
                label: block.label,
                stmts,
            });
        }

        result.extend(edges.materialize_splits(&labels));
        result
    }
}

fn insert_before_terminator(out: &mut Vec<Stmt>, body: Vec<Stmt>, inserts: Vec<Stmt>) {
    let term_pos = body.iter().rposition(|s| {
        matches!(
            s.inner,
            StmtInner::Jump(_) | StmtInner::CJump(_) | StmtInner::Return(_)
        )
    });

    match term_pos {
        Some(pos) => {
            out.extend(body[..pos].iter().cloned());
            out.extend(inserts);
            out.extend(body[pos..].iter().cloned());
        }
        None => {
            out.extend(body);
            out.extend(inserts);
        }
    }
}

// === Internal data structures ===

struct ParsedBlock {
    label: BlockLabel,
    phis: Vec<PhiStmt>,
    body: Vec<Stmt>,
}

impl ParsedBlock {
    fn from_block(block: &BasicBlock) -> Self {
        let mut phis = Vec::new();
        let mut body = Vec::new();

        for stmt in &block.stmts {
            match &stmt.inner {
                StmtInner::Phi(p) => phis.push(p.clone()),
                _ => body.push(stmt.clone()),
            }
        }

        Self {
            label: block.label.clone(),
            phis,
            body,
        }
    }
}

fn next_basic_block_id(labels: &[BlockLabel]) -> usize {
    labels
        .iter()
        .filter_map(|l| match l {
            BlockLabel::BasicBlock(n) => Some(*n + 1),
            _ => None,
        })
        .max()
        .unwrap_or(1)
}

struct SlotAllocator {
    slots: HashMap<usize, Operand>,
    allocas: Vec<Stmt>,
}

impl SlotAllocator {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
            allocas: Vec::new(),
        }
    }

    fn get_or_alloc(&mut self, phi_dst: &Operand, next_vreg: &mut usize) -> Operand {
        let vreg = phi_dst.vreg_index().expect("phi dest must be local");

        self.slots
            .entry(vreg)
            .or_insert_with(|| {
                let idx = *next_vreg;
                *next_vreg += 1;
                let ptr = LocalVariable::new(Dtype::ptr_to(phi_dst.dtype().clone()), idx, None);
                let slot = Operand::from(ptr);
                self.allocas.push(Stmt::as_alloca(slot.clone()));
                slot
            })
            .clone()
    }

    fn build_stores(&self, phis: &[PhiStmt], pred_label: &BlockLabel) -> Vec<Stmt> {
        let pred_key = pred_label.key();

        phis.iter()
            .filter_map(|phi| {
                let vreg = phi.dst.vreg_index().expect("phi dest must be local");
                let slot = self.slots.get(&vreg)?;

                let value = phi
                    .incomings
                    .iter()
                    .find(|(label, _)| label.key() == pred_key)
                    .map(|(_, v)| v.clone())
                    .unwrap_or_else(|| Operand::from(0));

                Some(Stmt::as_store(value, slot.clone()))
            })
            .collect()
    }

    fn into_allocas(self) -> Vec<Stmt> {
        self.allocas
    }
}

struct EdgeSplitter {
    splits: HashMap<(usize, usize), BlockLabel>,
    split_stores: HashMap<(usize, usize), Vec<Stmt>>,
    pending_inserts: HashMap<usize, Vec<Stmt>>,
    next_block_id: usize,
}

impl EdgeSplitter {
    fn new(next_block_id: usize) -> Self {
        Self {
            splits: HashMap::new(),
            split_stores: HashMap::new(),
            pending_inserts: HashMap::new(),
            next_block_id,
        }
    }

    fn split(&mut self, pred: usize, succ: usize, stores: Vec<Stmt>) {
        let label = BlockLabel::BasicBlock(self.next_block_id);
        self.next_block_id += 1;
        self.splits.insert((pred, succ), label);
        self.split_stores.insert((pred, succ), stores);
    }

    fn insert_at_pred(&mut self, pred: usize, stores: Vec<Stmt>) {
        self.pending_inserts.entry(pred).or_default().extend(stores);
    }

    fn split_count(&self) -> usize {
        self.splits.len()
    }

    fn patch_terminators(&self, blocks: &mut [ParsedBlock], labels: &[BlockLabel]) {
        for (&(pred, succ), new_label) in &self.splits {
            if let Some(term) = blocks[pred].body.last_mut() {
                let target_key = labels[succ].key();
                match &mut term.inner {
                    StmtInner::Jump(j) if j.target.key() == target_key => {
                        j.target = new_label.clone();
                    }
                    StmtInner::CJump(j) => {
                        if j.true_label.key() == target_key {
                            j.true_label = new_label.clone();
                        }
                        if j.false_label.key() == target_key {
                            j.false_label = new_label.clone();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn materialize_splits(mut self, labels: &[BlockLabel]) -> Vec<BasicBlock> {
        self.splits
            .into_iter()
            .map(|((pred, succ), new_label)| {
                let stores = self.split_stores.remove(&(pred, succ)).unwrap_or_default();
                let mut stmts = stores;
                stmts.push(Stmt::as_jump(labels[succ].clone()));
                BasicBlock {
                    label: new_label,
                    stmts,
                }
            })
            .collect()
    }
}
