use std::{cell::RefCell, rc::Rc, usize};

use serde::{Serialize, Deserialize};
pub type Idx = usize;

#[derive(Debug)]
pub struct Pos<N: Clone, E: Clone> {
    idx: Idx,
    graph: Rc<RefCell<Graph<N, E>>>,
}

impl<N: Clone, E: Clone> Clone for Pos<N, E> {
    fn clone(&self) -> Self {
        Pos {
            idx: self.idx.clone(),
            graph: self.graph.clone(),
        }
    }
} 

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node<N: Clone, E:Clone>
where  {
    param: N,
    edges: Vec<(Idx, E)>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Edge<E> {
    pub v: Idx,
    pub u: Idx,
    pub u_to_v: E,
    pub v_to_u: E,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Graph<N: Clone, E: Clone> {
    nodes: Vec<Node<N, E>>,
}

impl<N: Clone, E: Clone> From<Vec<Edge<E>>> for Graph<N, E> {
    fn from(edges: Vec<Edge<E>>) -> Self {
        let mut graph = Graph::<N, E>::new();
        for edge in edges {
            graph.add_edge(edge);
        }
        graph
    }
}

impl<N: Clone, E: Clone> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge<E>) {
        self.nodes[edge.u].edges.push((edge.v, edge.u_to_v));
        self.nodes[edge.v].edges.push((edge.u, edge.v_to_u));
    }

    pub fn get_node(&self, idx: Idx) -> Node<N, E> {
        self.nodes[idx].clone()
    }
}

impl<N: Clone, E: Clone> Pos<N, E> {
    pub fn new(graph: Rc<RefCell<Graph<N, E>>>, idx: Idx) -> Pos<N, E> {
        Pos {
            graph,
            idx,
        }
    }

    pub fn set_to(&mut self, idx: Idx) {
        self.idx = idx;
    }

    pub fn get_me(&self) -> Idx {
        self.idx
    }

    pub fn get_adjacent(&self) -> Vec<Idx> {
        let mut adjacent = Vec::<Idx>::new();
        for (v, _) in (*self.graph.borrow()).get_node(self.idx).edges {
            adjacent.push(v);
        }
        adjacent
    }
}