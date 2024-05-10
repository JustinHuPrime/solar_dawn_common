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

use crate::{stack::CargoList, vec2, EntityId};

/// Produce a component
///
/// Materials are drawn from the cargo holds in the stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Production {
    pub stack: EntityId,
    pub factory: EntityId,
    pub component: StackComponent,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackComponent {
    FuelTank,
    CargoHold,
    Engine,
    Gun,
    LaunchClamp,
    WarheadBus,
    Habitat,
    Miner,
    Factory,
    ArmourPlate,
}

/// Transfer materials from one stack's cargo holds to another stack's
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTransfer {
    pub stack: EntityId,
    pub destination: EntityId,
    pub amount: CargoList,
}

/// Transfer fuel from one stack's fuel tanks to another stack's
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelTransfer {
    pub stack: EntityId,
    pub destination: EntityId,
    pub amount: u64,
}

/// Reload a warhead mount using a warhead carried by the stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reload {
    pub stack: EntityId,
    pub mount: EntityId,
}

/// Repair components using a factory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryRepair {
    pub stack: EntityId,
    pub factory: EntityId,
    pub components: Vec<EntityId>,
}

/// Repair a component using a habitat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitatRepair {
    pub stack: EntityId,
    pub habitat: EntityId,
    pub component: EntityId,
}

/// Transfer components between rendezvoused stacks or to a new stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTransfer {
    pub stack: EntityId,
    pub destination: StackTransferTarget,
    pub components: Vec<EntityId>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackTransferTarget {
    Existing(EntityId),
    New(u64),
}

/// Launch a warhead from a mount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launch {
    pub stack: EntityId,
    pub mount: EntityId,
    pub delta: vec2::Displacement,
}

/// Shoot a gun at another stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shoot {
    pub stack: EntityId,
    pub gun: EntityId,
    pub target: EntityId,
}

/// Burn engines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Burn {
    pub stack: EntityId,
    pub delta: vec2::Displacement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
