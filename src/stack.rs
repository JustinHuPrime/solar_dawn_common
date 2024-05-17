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

    pub fuel_tanks: Vec<FuelTank>,
    pub cargo_holds: Vec<CargoHold>,
    pub engines: Vec<Engine>,
    pub guns: Vec<Gun>,
    pub launch_clamps: Vec<WarheadMount>,
    pub habitats: Vec<Habitat>,
    pub miners: Vec<Miner>,
    pub factories: Vec<Factory>,
    pub armour_plates: Vec<ArmourPlate>,
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
    #[derive(Clone)]
    FuelTank<mass = 1> {
        pub fuel: u64,
    }
}

component! {
    /// A cargo hold - holds non-fuel items
    ///
    /// 20 points of cargo capacity
    #[derive(Clone)]
    CargoHold<mass = 1> {
        pub inventory: CargoList,
    }
}
/// A collection of items held in a cargo hold
///
/// More-or-less an inventory, but also used in transfer orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoList {
    pub ice: u64,
    pub ore: u64,
    pub materials: u64,
    pub warheads: u64,
}

component! {
    /// An engine
    ///
    /// max 25 points of other mass / engine
    ///
    /// burn takes 1 point of fuel / engine needed to make up the TWR
    ///
    /// excess thrust allows for larger deltas
    #[derive(Clone)]
    Engine<mass = 5> {
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
    #[derive(Clone)]
    Gun<mass = 5> {
    }
}

component! {
    /// A warhead mount for holding ordnance
    ///
    /// During the ordnance phase, any number of clamps may be ordered to
    /// launch their held warhead (provide a delta-v of up to 1 hex/turn).
    /// Reloading requires an economic phase action.
    #[derive(Clone)]
    WarheadMount<mass = 1> {
        pub loaded: bool,
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
    #[derive(Clone)]
    Habitat<mass = 10> {
        pub owner: PlayerId
    }
}

component! {
    /// A miner for getting resources from minor planets
    ///
    /// At the start of each economic phase, produces the resources specified
    /// by the body's resource abundances
    #[derive(Clone)]
    Miner<mass = 10> {
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
    #[derive(Clone)]
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
    #[derive(Clone)]
    ArmourPlate<mass = 5> {
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
