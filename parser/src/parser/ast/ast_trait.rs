//use super::errors::AstErr;
use std::fmt::Display;

pub trait Ast<N>: Display {
    fn root(&self) -> &N;
    //fn diff(&self, other: &Self) -> Vec<Box<Self>>;
    fn from_node(seed: N) -> Self;
    fn new() -> Self;
}
