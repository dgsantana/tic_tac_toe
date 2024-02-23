use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marks that the entity is a cell and contains its location in grid.
#[derive(Component, Deserialize, Serialize, Deref)]
pub struct CellIndex(usize);

impl CellIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}