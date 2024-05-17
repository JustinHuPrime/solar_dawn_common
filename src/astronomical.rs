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

#[cfg(feature = "server")]
use crate::EntityIdGenerator;
use crate::{vec2, EntityId};

/// A major astronomical body
///
/// Represents any astronomical body that has gravity arrows; can't be landed
/// on
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug)]
pub struct MajorBody {
    pub name: String,
    pub id: EntityId,
    pub position: vec2::Position,
    pub radius: f64,
    pub colour: String,
}
impl MajorBody {
    #[cfg(feature = "server")]
    pub fn new(
        name: &str,
        id_generator: &mut EntityIdGenerator,
        position: vec2::Position,
        radius: f64,
        colour: &str,
    ) -> Self {
        Self {
            name: name.into(),
            id: id_generator.next().unwrap(),
            position,
            radius,
            colour: colour.into(),
        }
    }
}

/// A minor astronomical body
///
/// Represents any astronomical body that doesn't have gravity arrows; may be
/// landed on
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug)]
pub struct MinorBody {
    pub name: String,
    pub id: EntityId,
    pub position: vec2::Position,
    pub radius: f64,
    pub ice_abundance: u64,
    pub ore_abundance: u64,
}
impl MinorBody {
    #[cfg(feature = "server")]
    pub fn new(
        name: &str,
        id_generator: &mut EntityIdGenerator,
        position: vec2::Position,
        radius: f64,
        ice_abundance: u64,
        ore_abundance: u64,
    ) -> Self {
        Self {
            name: name.into(),
            id: id_generator.next().unwrap(),
            position,
            radius,
            ice_abundance,
            ore_abundance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
