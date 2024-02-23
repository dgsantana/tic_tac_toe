use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// An event that indicates a symbol pick.
///
/// We don't replicate the whole UI, so we can't just send the picked entity because on server it may be different.
/// So we send the cell location in grid and calculate the entity on server based on this.
#[derive(Clone, Copy, Deserialize, Event, Serialize)]
pub struct CellPick(usize);

impl CellPick {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}
