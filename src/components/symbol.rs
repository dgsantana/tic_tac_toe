use bevy::prelude::*;
use bevy_replicon::replicon_core::replication_rules::Replication;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::CellIndex;

/// A component that defines the symbol of a player or a filled cell.
#[derive(Clone, Component, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Symbol {
    #[default]
    Cross,
    Nought,
}

impl Symbol {
    pub fn glyph(self) -> &'static str {
        match self {
            Symbol::Cross => "❌",
            Symbol::Nought => "⭕",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Symbol::Cross => Color::rgb(1.0, 0.5, 0.5),
            Symbol::Nought => Color::rgb(0.5, 0.5, 1.0),
        }
    }

    pub fn next(self) -> Self {
        match self {
            Symbol::Cross => Symbol::Nought,
            Symbol::Nought => Symbol::Cross,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Cross => f.write_str("cross"),
            Symbol::Nought => f.write_str("nought"),
        }
    }
}

#[derive(Bundle)]
pub struct SymbolBundle {
    symbol: Symbol,
    cell_index: CellIndex,
    replication: Replication,
}

impl SymbolBundle {
    pub fn new(symbol: Symbol, index: usize) -> Self {
        Self {
            cell_index: CellIndex::new(index),
            symbol,
            replication: Replication,
        }
    }
}
