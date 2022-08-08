// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_span::Symbol;

use indexmap::{IndexMap, IndexSet};

/// A directed graph describing the caller-callee relationships of the program.
/// A node corresponds to a function.
/// A directed edge of the form `a --> b` corresponds to an invocation of function `b` in the body of `a`.
pub struct CallGraph {
    /// The source nodes in the call graph. These correspond to "program functions".
    sources: Vec<Symbol>,
    // TODO: Better name.
    /// The directed edges in the call graph.
    edges: IndexMap<Symbol, Vec<Symbol>>,
}

impl CallGraph {
    /// Initializes a new `CallGraph` from a vector of source nodes.
    pub fn new(sources: Vec<Symbol>) -> Self {
        Self {
            sources,
            edges: IndexMap::new(),
        }
    }

    /// Adds an edge to the call graph.
    pub fn add_edge(&mut self, from: Symbol, to: Symbol) {
        let entry = self.edges.entry(from).or_default();
        entry.push(to)
    }

    /// Detects if there is a cycle in the call graph.
    pub fn contains_cycle(&self) -> bool {
        // TODO: Init with capacity
        let mut seen: IndexSet<Symbol> = IndexSet::new();

        // Add all the source nodes the`fringe`.
        let mut fringe = self.sources.clone();

        while !fringe.is_empty() {
            // Note that this unwrap is safe since `fringe` is not empty.
            let node = fringe.pop().unwrap();

            // If `seen` contains `node`, then a cycle exists.
            if seen.contains(&node) {
                return true;
            } else {
                seen.insert(node);
            }

            // Add the children of `node` to the `fringe`.
            if let Some(children) = self.edges.get(&node) {
                fringe.extend_from_slice(children)
            }
        }

        // No cycle was detected.
        false
    }
}
