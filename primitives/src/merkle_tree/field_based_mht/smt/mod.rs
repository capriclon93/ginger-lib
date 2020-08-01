#![allow(dead_code)]

mod parameters;
mod big_merkle_tree;
mod big_lazy_merkle_tree;
pub mod error;

use crate::{FieldBasedHashParameters, PoseidonHash};
use crate::crh::poseidon::parameters::{MNT4753PoseidonParameters, MNT6753PoseidonParameters};

use algebra::fields::mnt6753::Fr as MNT6753Fr;
use algebra::fields::mnt4753::Fr as MNT4753Fr;
use algebra::PrimeField;

use serde::{Serialize,Deserialize};

pub type MNT4PoseidonHash = PoseidonHash<MNT4753Fr, MNT4753PoseidonParameters>;
pub type MNT6PoseidonHash = PoseidonHash<MNT6753Fr, MNT6753PoseidonParameters>;

pub trait SmtPoseidonParameters: 'static + FieldBasedHashParameters {
    // The arity of the Sparse Merkle Tree
    const MERKLE_ARITY: usize;
    // The pre-computed hashes of the empty nodes for the different levels of the SMT
    const EMPTY_HASH_CST: &'static [Self::Fr];
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ActionLeaf {
    Insert,
    Remove,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
// Coordinates system for identifying a node
pub struct Coord {
    // height in the Merkle tree (0 -> leaves)
    height: usize,
    // the index of the node in that level
    idx: usize,
}

impl Coord {
    pub fn new(height: usize, idx: usize) -> Self {
        Self { height, idx }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
// Action associated to the leaf
pub struct OperationLeaf <F: PrimeField>{
    coord: Coord,
    action: ActionLeaf,
    hash: Option<F>,
}

impl<F: PrimeField> OperationLeaf<F> {
    pub fn new(height: usize, idx: usize, action: ActionLeaf, hash: Option<F>) -> Self {
        Self { coord: Coord { height, idx }, action, hash}
    }
}

