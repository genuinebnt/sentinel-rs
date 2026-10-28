use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssetKind {
    Host,
}

impl std::fmt::Display for AssetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Host => "host",
        };

        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationKind {
    SubDomainOf,
}

impl std::fmt::Display for RelationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::SubDomainOf => "subdomain_of",
        };

        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
    Both,
}

impl Direction {
    pub const fn reversed(self) -> Self {
        match self {
            Self::Out => Self::In,
            Self::In => Self::Out,
            Self::Both => Self::Both,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::In => "in",
            Self::Out => "out",
            Self::Both => "both",
        };

        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Asset {
    Host { hostname: String },
}

impl Asset {
    fn host(host: &str) -> Self {
        Self::Host {
            hostname: host.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Relation {
    pub src: Asset,
    pub kind: RelationKind,
    pub dst: Asset,
}

#[derive(Debug, Default)]
pub struct AssetGraph {
    pub nodes: Vec<
}

impl AssetGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(&id)
    }

    pub fn add_node(&mut self, node: Node) {
        if self.nodes.contains_key(&node)

    }
}
