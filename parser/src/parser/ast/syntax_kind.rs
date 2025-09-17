use std::hash::Hash;
use std::fmt::Debug;

/// Nodes that the ast can have.
pub trait SyntaxKind: Debug + Sized + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Default {
}
