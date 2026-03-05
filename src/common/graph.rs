use std::collections::{HashMap, HashSet, VecDeque};

pub trait CfgNode {
    fn label(&self) -> Option<String>;
    fn successors(
        &self,
        idx: usize,
        num_nodes: usize,
        label_map: &HashMap<String, usize>,
    ) -> Vec<usize>;
}

pub struct Graph {
    succs: Vec<Vec<usize>>,
    preds: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(succs: Vec<Vec<usize>>) -> Self {
        let n = succs.len();
        let mut preds = vec![Vec::new(); n];
        for (i, succ_list) in succs.iter().enumerate() {
            for &s in succ_list {
                preds[s].push(i);
            }
        }
        Self { succs, preds }
    }

    pub fn from_nodes<N: CfgNode>(nodes: &[N]) -> Self {
        let n = nodes.len();
        let label_map: HashMap<String, usize> = nodes
            .iter()
            .enumerate()
            .filter_map(|(i, node)| node.label().map(|k| (k, i)))
            .collect();
        let succs = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| node.successors(i, n, &label_map))
            .collect();
        Self::new(succs)
    }

    pub fn num_nodes(&self) -> usize {
        self.succs.len()
    }

    pub fn successors(&self, node: usize) -> &[usize] {
        &self.succs[node]
    }

    pub fn predecessors(&self, node: usize) -> &[usize] {
        &self.preds[node]
    }

    pub fn succs_vec(&self) -> &[Vec<usize>] {
        &self.succs
    }

    pub fn preds_vec(&self) -> &[Vec<usize>] {
        &self.preds
    }
}

pub trait Lattice: Clone + PartialEq {
    fn bottom() -> Self;
    fn join(&mut self, other: &Self);
    fn transfer(gen: &Self, kill: &Self, out: &Self) -> Self;
}

impl Lattice for bool {
    fn bottom() -> Self {
        false
    }

    fn join(&mut self, other: &Self) {
        *self = *self || *other;
    }

    fn transfer(gen: &Self, kill: &Self, out: &Self) -> Self {
        *gen || (*out && !*kill)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct VregSet(pub HashSet<usize>);

impl Lattice for VregSet {
    fn bottom() -> Self {
        VregSet(HashSet::new())
    }

    fn join(&mut self, other: &Self) {
        self.0.extend(other.0.iter().copied());
    }

    fn transfer(gen: &Self, kill: &Self, out: &Self) -> Self {
        let mut result = gen.0.clone();
        for v in &out.0 {
            if !kill.0.contains(v) {
                result.insert(*v);
            }
        }
        VregSet(result)
    }
}

pub struct BackwardLiveness<L> {
    pub live_in: Vec<L>,
    pub live_out: Vec<L>,
}

impl<L: Lattice> BackwardLiveness<L> {
    pub fn compute(gen: &[L], kill: &[L], graph: &Graph) -> Self {
        let n = graph.num_nodes();

        let mut live_in: Vec<L> = (0..n).map(|_| L::bottom()).collect();
        let mut live_out: Vec<L> = (0..n).map(|_| L::bottom()).collect();

        let mut in_worklist = vec![true; n];
        let mut worklist: VecDeque<usize> = (0..n).rev().collect();

        while let Some(i) = worklist.pop_front() {
            in_worklist[i] = false;

            let mut new_out = L::bottom();
            for &s in graph.successors(i) {
                new_out.join(&live_in[s]);
            }

            let new_in = L::transfer(&gen[i], &kill[i], &new_out);

            if new_in != live_in[i] {
                live_in[i] = new_in;
                live_out[i] = new_out;

                for &p in graph.predecessors(i) {
                    if !in_worklist[p] {
                        in_worklist[p] = true;
                        worklist.push_back(p);
                    }
                }
            } else {
                live_out[i] = new_out;
            }
        }

        Self { live_in, live_out }
    }
}
