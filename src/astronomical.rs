// Copyright 2024 Justin Hu
//
// This file is part of Solar Dawn.
//
// Solar Dawn is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Solar Dawn is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License
// for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Solar Dawn. If not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};

use crate::{vec2, EntityId};

/// A major astronomical body
///
/// Represents any astronomical body that has gravity arrows; can't be landed
/// on
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorBody {
    pub name: String,
    pub id: EntityId,
    pub position: vec2::Position,
    pub radius: f64,
}

/// A minor astronomical body
///
/// Represents any astronomical body that doesn't have gravity arrows; may be
/// landed on
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinorBody {
    pub name: String,
    pub id: EntityId,
    pub position: vec2::Position,
    pub radius: f64,
    pub ice_abundance: u64,
    pub ore_abundance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
