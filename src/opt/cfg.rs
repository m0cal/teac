use crate::common::graph::{CfgNode, Graph};
use crate::ir::function::{BasicBlock, BlockLabel};
use crate::ir::stmt::StmtInner;
use std::collections::HashMap;

impl CfgNode for BasicBlock {
    fn label(&self) -> Option<String> {
        Some(self.label.key())
    }

    fn successors(
        &self,
        idx: usize,
        num_nodes: usize,
        label_map: &HashMap<String, usize>,
    ) -> Vec<usize> {
        let term = self.stmts.last();

        match term.map(|s| &s.inner) {
            Some(StmtInner::Jump(j)) => vec![label_map[&j.target.key()]],
            Some(StmtInner::CJump(j)) => vec![
                label_map[&j.true_label.key()],
                label_map[&j.false_label.key()],
            ],
            Some(StmtInner::Return(_)) => Vec::new(),
            _ => {
                if idx + 1 < num_nodes {
                    vec![idx + 1]
                } else {
                    Vec::new()
                }
            }
        }
    }
}

pub struct Cfg {
    labels: Vec<BlockLabel>,
    graph: Graph,
}

impl Cfg {
    pub fn from_blocks(blocks: &[BasicBlock]) -> Self {
        let labels: Vec<BlockLabel> = blocks.iter().map(|b| b.label.clone()).collect();
        let graph = Graph::from_nodes(blocks);
        Self { labels, graph }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn num_blocks(&self) -> usize {
        self.graph.num_nodes()
    }

    pub fn successors(&self, block: usize) -> &[usize] {
        self.graph.successors(block)
    }

    pub fn predecessors(&self, block: usize) -> &[usize] {
        self.graph.predecessors(block)
    }

    pub fn label(&self, block: usize) -> &BlockLabel {
        &self.labels[block]
    }

    pub fn labels(&self) -> &[BlockLabel] {
        &self.labels
    }
}
