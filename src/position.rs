/*
Copyright 2024 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::fmt;
use std::hash::Hash;

/// A position in a language term tree.  See [`TreePosition`].
pub type PositionInLanguageTerm = TreePosition;
/// A position in the strategy tree used by the traced executor.  See [`TreePosition`].
pub type PositionInRewriteProcess = TreePosition;

/// A position in a language term, encoded as a sequence of child indices from the root.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TreePosition {
    absolute_coordinates_from_root: Vec<usize>,
}

impl TreePosition {
    /// Constructs a position from a precomputed coordinate vector.
    pub fn from_absolute_coordinates(absolute_coordinates_from_root: Vec<usize>) -> Self {
        Self {
            absolute_coordinates_from_root,
        }
    }

    /// Returns the parent position, or `None` if `self` is the root.
    pub fn get_parent_position(&self) -> Option<Self> {
        if self.absolute_coordinates_from_root.is_empty() {
            None
        } else {
            let depth = self.get_depth();
            let parent = Self::from_absolute_coordinates(
                self.absolute_coordinates_from_root[0..(depth - 1)].to_vec(),
            );
            Some(parent)
        }
    }

    /// Returns the number of steps from the root (0 for the root itself).
    pub fn get_depth(&self) -> usize {
        self.absolute_coordinates_from_root.len()
    }

    /// Returns the root position (the empty coordinate sequence, written ε).
    pub fn get_root_position() -> Self {
        PositionInLanguageTerm {
            absolute_coordinates_from_root: vec![],
        }
    }

    /// Returns the position of the `n`-th child.
    ///
    /// If `self` is position `p`, then `n.p` is the position of the `n`-th
    /// argument: if `x` is at `p` in `y`, then `x` is at `1.p` in `z(y, ·, ·)`,
    /// at `2.p` in `z(·, y, ·)`, etc.
    pub fn get_position_of_nth_child(&self, n: usize) -> Self {
        let mut absolute_coords = self.absolute_coordinates_from_root.clone();
        absolute_coords.push(n);
        Self::from_absolute_coordinates(absolute_coords)
    }

    /// Returns the coordinate slice, e.g. `&[1, 2]` for position `1_2`.
    pub fn get_absolute_coordinates_from_root(&self) -> &[usize] {
        &self.absolute_coordinates_from_root
    }
}

impl fmt::Display for TreePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.absolute_coordinates_from_root.is_empty() {
            write!(f, "ε")
        } else {
            let as_strs: Vec<String> = self
                .absolute_coordinates_from_root
                .iter()
                .map(|x| x.to_string())
                .collect();
            write!(f, "{:}", as_strs.join("_"))
        }
    }
}
