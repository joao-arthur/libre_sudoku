use crate::{
    cell::Cell,
    group::{Group, group_from_str},
    solved_group::SolvedGroup,
};
use std::fmt;

pub type SolvedBoard = [SolvedGroup; 9];

#[cfg(test)]
mod tests {}
