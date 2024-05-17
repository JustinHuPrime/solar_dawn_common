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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::EntityIdGenerator;
use crate::{vec2, EntityId, PlayerId};

/// A stack
///
/// Anything that's not an astronomical body or a warhead
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug)]
pub struct Stack {
    pub name: String,
    pub id: EntityId,
    pub position: vec2::Position,
    pub velocity: vec2::Displacement,
    pub owner: PlayerId,

    pub fuel_tanks: HashMap<EntityId, FuelTank>,
    pub cargo_holds: HashMap<EntityId, CargoHold>,
    pub engines: HashMap<EntityId, Engine>,
    pub guns: HashMap<EntityId, Gun>,
    pub launch_clamps: HashMap<EntityId, WarheadMount>,
    pub habitats: HashMap<EntityId, Habitat>,
    pub miners: HashMap<EntityId, Miner>,
    pub factories: HashMap<EntityId, Factory>,
    pub armour_plates: HashMap<EntityId, ArmourPlate>,
}
impl Stack {
    #[cfg(feature = "server")]
    pub fn new(
        name: &str,
        id_generator: &mut EntityIdGenerator,
        position: vec2::Position,
        velocity: vec2::Displacement,
        owner: PlayerId,
    ) -> Self {
        Self {
            name: name.into(),
            id: id_generator.next().unwrap(),
            position,
            velocity,
            owner,

            fuel_tanks: HashMap::new(),
            cargo_holds: HashMap::new(),
            engines: HashMap::new(),
            guns: HashMap::new(),
            launch_clamps: HashMap::new(),
            habitats: HashMap::new(),
            miners: HashMap::new(),
            factories: HashMap::new(),
            armour_plates: HashMap::new(),
        }
    }
}

/// Create a component type
///
/// `name` = component name
/// `fields` = extra fields
macro_rules! component {
    ( $(#[$attributes:meta])* $name:ident<mass = $mass:literal> { $($fields:tt)* } ) => {
        $(#[$attributes])*
        #[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
        #[cfg_attr(feature = "server", derive(Serialize))]
        #[derive(Debug)]
        pub struct $name {
            pub id: EntityId,
            pub damaged: bool,
            $($fields)*
        }
        impl $name {
            const MASS: u64 = $mass;
        }
    };
}

component! {
    /// A fuel tank - holds fuel
    ///
    /// 20 points of fuel capacity
    FuelTank<mass = 1> {
        pub fuel: u64,
    }
}
impl FuelTank {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
            fuel: 0,
        }
    }
}

component! {
    /// A cargo hold - holds non-fuel items
    ///
    /// 20 points of cargo capacity
    CargoHold<mass = 1> {
        pub inventory: CargoList,
    }
}
impl CargoHold {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
            inventory: CargoList::new(0, 0, 0, 0),
        }
    }
}
/// A collection of items held in a cargo hold
///
/// More-or-less an inventory, but also used in transfer orders
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct CargoList {
    pub ice: u64,
    pub ore: u64,
    pub materials: u64,
    pub warheads: u64,
}
impl CargoList {
    pub fn new(ice: u64, ore: u64, materials: u64, warheads: u64) -> Self {
        Self {
            ice,
            ore,
            materials,
            warheads,
        }
    }
}

component! {
    /// An engine
    ///
    /// max 25 points of other mass / engine
    ///
    /// burn takes 1 point of fuel / engine needed to make up the TWR
    ///
    /// excess thrust allows for larger deltas
    Engine<mass = 5> {
    }
}
impl Engine {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
        }
    }
}

component! {
    /// A gun (direct-fire)
    ///
    /// During the combat phase, can shoot at other stacks; damages one
    /// component in the targetted stack if it hits; has 2/3 hit chance at 1
    /// hex away, guaranteed to hit at 0 hexes away; hit chance follows the
    /// inverse-fourth-power relationship (2 hexes = (2/3)^4 = ~0.20 hit
    /// chance)
    Gun<mass = 5> {
    }
}
impl Gun {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
        }
    }
}

component! {
    /// A warhead mount for holding ordnance
    ///
    /// During the ordnance phase, any number of clamps may be ordered to
    /// launch their held warhead (provide a delta-v of up to 1 hex/turn).
    /// Reloading requires an economic phase action.
    WarheadMount<mass = 1> {
        pub loaded: bool,
    }
}
impl WarheadMount {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
            loaded: false,
        }
    }
}

component! {
    /// A habitat for holding humans
    ///
    /// During the economic phase, can repair up to one item per habitat at a
    /// cost of 1 point of materials
    ///
    /// Additionally serves as a source of control; you gain control of
    /// anything in the same stack as one of your habitats
    Habitat<mass = 10> {
        pub owner: PlayerId
    }
}
impl Habitat {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator, owner: PlayerId) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
            owner,
        }
    }
}

component! {
    /// A miner for getting resources from minor planets
    ///
    /// At the start of each economic phase, produces the resources specified
    /// by the body's resource abundances
    Miner<mass = 10> {
    }
}
impl Miner {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
        }
    }
}

component! {
    /// A factory
    ///
    /// During the economic phase, a factory may make one action from the
    /// following list:
    ///  - produce a component (at 1 mass = 1 materials)
    ///  - produce any number of warheads from materials at 5:1
    ///  - convert any amount of ore into materials at 2:1
    ///  - convert any amount of ice into fuel at 2:1
    ///  - repair any number of components in a single stack (costs 1 point of
    ///    materials per component repaired)
    Factory<mass = 50> {
    }
}
impl Factory {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
        }
    }
}

component! {
    /// An armour plate - always damaged first (but not necessarily destroyed
    /// first)
    ArmourPlate<mass = 5> {
    }
}
impl ArmourPlate {
    #[cfg(feature = "server")]
    pub fn new(id_generator: &mut EntityIdGenerator) -> Self {
        Self {
            id: id_generator.next().unwrap(),
            damaged: false,
        }
    }
}

/// A warhead
///
/// Deals 5 points of damage
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Warhead {
    pub id: EntityId,
    pub position: vec2::Position,
    pub velocity: vec2::Displacement,
    pub owner: PlayerId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
