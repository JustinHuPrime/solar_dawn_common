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

//! Common library for Solar Dawn
//!
//! Both the server and the client interact with this library to ensure
//! consistent game mechanics implementation
//!
//! Solar Dawn is a simultaneous-turn-resolution space 4X featuring logistics,
//! customizable and modular ships and missiles, and a Triplanetary-inspired
//! movement system
//!
//! # Turn structure
//!
//! A turn is split into four phases; the game state is displayed before each
//! phase, and orders are received from all players to be resolved
//! simultaneously.
//!
//! 1. The economic phase is when production, cargo transfer, fuel transfer,
//!    reload, repair, and stack transfer orders are issued and resolved
//! 2. The ordnance phase is when ordnance launching and arming orders are
//!    issued and resolved
//! 3. The combat phase is when direct-fire weapons are issued and resolved
//! 4. The movement phase is when movement orders are issued and resolved

#![forbid(unsafe_code)]

use std::collections::HashMap;

use astronomical::{MajorBody, MinorBody};
#[cfg(feature = "server")]
use rand::SeedableRng;
#[cfg(feature = "server")]
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use stack::{Stack, Warhead};

pub mod astronomical;
pub mod order;
pub mod stack;
pub mod vec2;

/// The current phase within the round
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug, Clone, Copy)]
pub enum Phase {
    Economic,
    Ordnance,
    Combat,
    Movement,
}

/// The state of the game
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug)]
pub struct GameState {
    pub major_bodies: HashMap<EntityId, MajorBody>,
    pub minor_bodies: HashMap<EntityId, MinorBody>,
    pub stacks: HashMap<EntityId, Stack>,
    pub warheads: HashMap<EntityId, Warhead>,
    pub phase: Phase,
}
impl GameState {
    #[cfg(feature = "server")]
    /// Generate a new game with random solar system configuration
    ///
    /// 1 hex = 1/16 AU
    pub fn new(
        seed: &<ChaCha20Rng as SeedableRng>::Seed,
        num_players: u8,
        id_generator: &mut EntityIdGenerator,
    ) -> Self {
        use std::{
            cmp::{max, min},
            f64::consts::{PI, TAU},
        };

        use rand::distributions::{weighted::WeightedIndex, Distribution, Uniform};

        use crate::stack::{CargoHold, Factory, FuelTank, Habitat};

        assert!((2..=6).contains(&num_players), "did not have 2-6 players");

        let mut rng = ChaCha20Rng::from_seed(*seed);

        let mut major_bodies = HashMap::new();
        let mut minor_bodies = HashMap::new();
        let mut stacks = HashMap::new();

        // generate major bodies
        let angle_distribution = Uniform::from(0.0..TAU);

        // sol
        let sol = MajorBody::new(
            "Sol",
            id_generator,
            vec2::Position::new(0, 0),
            0.8,
            "#ffff00",
        );
        major_bodies.insert(sol.id, sol);

        // mercury
        let mercury_angle = angle_distribution.sample(&mut rng);
        let mercury = MajorBody::new(
            "Mercury",
            id_generator,
            (6.0 * mercury_angle.cos(), 6.0 * mercury_angle.sin()).into(),
            0.3,
            "#404040",
        );
        major_bodies.insert(mercury.id, mercury);

        // venus
        let venus_angle = angle_distribution.sample(&mut rng);
        let venus = MajorBody::new(
            "Venus",
            id_generator,
            (12.0 * venus_angle.cos(), 12.0 * venus_angle.sin()).into(),
            0.6,
            "#ffc000",
        );
        major_bodies.insert(venus.id, venus);

        // terra + luna - always at 3 o'clock
        let terra = MajorBody::new("Terra", id_generator, (16.0, 0.0).into(), 0.6, "#0000ff");
        let luna = MajorBody::new(
            "Luna",
            id_generator,
            terra.position + vec2::Displacement::new(3, 2),
            0.4,
            "#808080",
        );
        let terra_position = terra.position;
        major_bodies.insert(terra.id, terra);
        major_bodies.insert(luna.id, luna);

        // mars
        let mars_angle = angle_distribution.sample(&mut rng);
        let mars = MajorBody::new(
            "Mars",
            id_generator,
            (24.0 * mars_angle.cos(), 24.0 * mars_angle.sin()).into(),
            0.5,
            "#ff0000",
        );
        let mars_position = mars.position;
        major_bodies.insert(mars.id, mars);

        // jupiter + moons
        let jupiter_angle = angle_distribution.sample(&mut rng);
        let jupiter = MajorBody::new(
            "Jupiter",
            id_generator,
            (40.0 * jupiter_angle.cos(), 40.0 * jupiter_angle.sin()).into(),
            0.8,
            "#ffc000",
        );
        let europa = MajorBody::new(
            "Europa",
            id_generator,
            jupiter.position + vec2::Displacement::new(0, 3),
            0.3,
            "#a0a0ff",
        );
        let callisto = MajorBody::new(
            "Callisto",
            id_generator,
            jupiter.position + vec2::Displacement::new(-4, 0),
            0.3,
            "#404040",
        );
        let ganymede = MajorBody::new(
            "Ganymede",
            id_generator,
            jupiter.position + vec2::Displacement::new(4, -2),
            0.3,
            "#404040",
        );
        major_bodies.insert(jupiter.id, jupiter);
        major_bodies.insert(europa.id, europa);
        major_bodies.insert(callisto.id, callisto);
        major_bodies.insert(ganymede.id, ganymede);

        // generate minor bodies

        // phobos, deimos
        let phobos = MinorBody::new(
            "Phobos",
            id_generator,
            mars_position + vec2::Displacement::new(0, -2),
            0.2,
            1,
            0,
        );
        let deimos = MinorBody::new(
            "Deimos",
            id_generator,
            mars_position + vec2::Displacement::new(3, 0),
            0.2,
            1,
            0,
        );
        minor_bodies.insert(phobos.id, phobos);
        minor_bodies.insert(deimos.id, deimos);

        struct AsteroidNameGenerator {
            last: u64,
        }
        impl AsteroidNameGenerator {
            const STEP: u64 = 13121;
            pub fn new() -> Self {
                Self { last: 0 }
            }
            pub fn next(&mut self) -> String {
                self.last += Self::STEP;
                self.last %= 60_000;
                format!("{}", self.last + 10_000)
            }
        }

        let mut asteroid_name_generator = AsteroidNameGenerator::new();

        // asteroid belt = radius 29 - 36
        let resource_values = [0, 1, 2, 3, 4, 5, 6];
        let resource_index_distribution = WeightedIndex::new([7, 6, 5, 4, 3, 2, 1]).unwrap();
        for q in -36_i64..=36 {
            for r in max(-36, -q - 36)..=min(36, -q + 36) {
                if (q.unsigned_abs() + r.unsigned_abs() + (q + r).unsigned_abs()) / 2 < 29 {
                    continue;
                }
                let ice_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                let ore_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                if ice_abundance == 0 && ore_abundance == 0 {
                    continue;
                }
                let asteroid = MinorBody::new(
                    &asteroid_name_generator.next(),
                    id_generator,
                    vec2::Position::new(q, r),
                    0.2,
                    ice_abundance,
                    ore_abundance,
                );
                minor_bodies.insert(asteroid.id, asteroid);
            }
        }

        // trojans
        for distance in 38..=42 {
            for step in -15..=15 {
                let ice_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                let ore_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                if ice_abundance == 0 && ore_abundance == 0 {
                    continue;
                }

                let angle_delta = step as f64 / 180.0 * PI;
                let angle = jupiter_angle + angle_delta + PI / 3.0;
                let position =
                    (distance as f64 * angle.cos(), distance as f64 * angle.sin()).into();
                if minor_bodies
                    .iter()
                    .any(|(_, body)| body.position == position)
                {
                    continue;
                }

                let asteroid = MinorBody::new(
                    &asteroid_name_generator.next(),
                    id_generator,
                    position,
                    0.2,
                    ice_abundance,
                    ore_abundance,
                );
                minor_bodies.insert(asteroid.id, asteroid);
            }
        }

        // greeks
        for distance in 38..=42 {
            for step in -15..=15 {
                let ice_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                let ore_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                if ice_abundance == 0 && ore_abundance == 0 {
                    continue;
                }

                let angle_delta = step as f64 / 180.0 * PI;
                let angle = jupiter_angle + angle_delta - PI / 3.0;
                let position =
                    (distance as f64 * angle.cos(), distance as f64 * angle.sin()).into();
                if minor_bodies
                    .iter()
                    .any(|(_, body)| body.position == position)
                {
                    continue;
                }

                let asteroid = MinorBody::new(
                    &asteroid_name_generator.next(),
                    id_generator,
                    position,
                    0.2,
                    ice_abundance,
                    ore_abundance,
                );
                minor_bodies.insert(asteroid.id, asteroid);
            }
        }

        // additional hildas
        for distance in 32..38 {
            for step in -15..=15 {
                let ice_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                let ore_abundance = resource_values[resource_index_distribution.sample(&mut rng)];
                if ice_abundance == 0 && ore_abundance == 0 {
                    continue;
                }

                let angle_delta = step as f64 / 180.0 * PI;
                let angle = jupiter_angle + angle_delta + PI;
                let position =
                    (distance as f64 * angle.cos(), distance as f64 * angle.sin()).into();
                if minor_bodies
                    .iter()
                    .any(|(_, body)| body.position == position)
                {
                    continue;
                }

                let asteroid = MinorBody::new(
                    &asteroid_name_generator.next(),
                    id_generator,
                    position,
                    0.2,
                    ice_abundance,
                    ore_abundance,
                );
                minor_bodies.insert(asteroid.id, asteroid);
            }
        }

        // generate starting stacks
        const STARTING_STATION_NAMES: [&str; 6] = [
            "Space Station Freedom",
            "Mir",
            "Tiangong",
            "Bharatiya Antariksha Station",
            "Tokyo Gateway",
            "Berlin Highport",
        ];
        let starting_station_orbital_elements = match num_players {
            2 => vec![
                (
                    vec2::Displacement::new(0, -1),
                    vec2::Displacement::new(1, 1),
                ),
                (
                    vec2::Displacement::new(0, 1),
                    vec2::Displacement::new(-1, -1),
                ),
            ],
            3 => vec![
                (
                    vec2::Displacement::new(0, -1),
                    vec2::Displacement::new(1, 1),
                ),
                (
                    vec2::Displacement::new(1, 1),
                    vec2::Displacement::new(-1, 0),
                ),
                (
                    vec2::Displacement::new(-1, 0),
                    vec2::Displacement::new(0, -1),
                ),
            ],
            4 => vec![
                (
                    vec2::Displacement::new(0, -1),
                    vec2::Displacement::new(1, 1),
                ),
                (vec2::Displacement::new(1, 0), vec2::Displacement::new(0, 1)),
                (
                    vec2::Displacement::new(0, 1),
                    vec2::Displacement::new(-1, -1),
                ),
                (
                    vec2::Displacement::new(-1, 0),
                    vec2::Displacement::new(0, -1),
                ),
            ],
            5 => vec![
                (
                    vec2::Displacement::new(0, -1),
                    vec2::Displacement::new(1, 1),
                ),
                (vec2::Displacement::new(1, 0), vec2::Displacement::new(0, 1)),
                (
                    vec2::Displacement::new(1, 1),
                    vec2::Displacement::new(-1, 0),
                ),
                (
                    vec2::Displacement::new(0, 1),
                    vec2::Displacement::new(-1, -1),
                ),
                (
                    vec2::Displacement::new(-1, 0),
                    vec2::Displacement::new(0, -1),
                ),
            ],
            6 => vec![
                (
                    vec2::Displacement::new(0, -1),
                    vec2::Displacement::new(1, 1),
                ),
                (vec2::Displacement::new(1, 0), vec2::Displacement::new(0, 1)),
                (
                    vec2::Displacement::new(1, 1),
                    vec2::Displacement::new(-1, 0),
                ),
                (
                    vec2::Displacement::new(0, 1),
                    vec2::Displacement::new(-1, -1),
                ),
                (
                    vec2::Displacement::new(-1, 0),
                    vec2::Displacement::new(0, -1),
                ),
                (
                    vec2::Displacement::new(-1, -1),
                    vec2::Displacement::new(1, 0),
                ),
            ],
            _ => panic!("unexpected number of players"),
        };
        for player in 0..num_players {
            let mut station = Stack::new(
                STARTING_STATION_NAMES[player as usize],
                id_generator,
                terra_position + starting_station_orbital_elements[player as usize].0,
                starting_station_orbital_elements[player as usize].1,
                player.into(),
            );

            let factory = Factory::new(id_generator);
            station.factories.insert(factory.id, factory);

            let habitat = Habitat::new(id_generator, player.into());
            station.habitats.insert(habitat.id, habitat);

            let mut fuel_tank = FuelTank::new(id_generator);
            fuel_tank.fuel = 20;
            station.fuel_tanks.insert(fuel_tank.id, fuel_tank);
            let mut fuel_tank = FuelTank::new(id_generator);
            fuel_tank.fuel = 20;
            station.fuel_tanks.insert(fuel_tank.id, fuel_tank);

            let mut cargo_hold = CargoHold::new(id_generator);
            cargo_hold.inventory.materials = 20;
            station.cargo_holds.insert(cargo_hold.id, cargo_hold);
            let mut cargo_hold = CargoHold::new(id_generator);
            cargo_hold.inventory.materials = 20;
            station.cargo_holds.insert(cargo_hold.id, cargo_hold);
            let mut cargo_hold = CargoHold::new(id_generator);
            cargo_hold.inventory.materials = 20;
            station.cargo_holds.insert(cargo_hold.id, cargo_hold);

            stacks.insert(station.id, station);
        }

        Self {
            major_bodies,
            minor_bodies,
            stacks,
            warheads: HashMap::new(),
            phase: Phase::Economic,
        }
    }
}

/// A player ID
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u8);
impl From<u8> for PlayerId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<PlayerId> for u8 {
    fn from(value: PlayerId) -> Self {
        value.0
    }
}

/// An entity ID
///
/// This is technically namespaced, but that isn't exposed at the type level
#[cfg_attr(
    any(feature = "client", feature = "server"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(u64);
impl From<u64> for EntityId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<EntityId> for u64 {
    fn from(value: EntityId) -> Self {
        value.0
    }
}

/// An entity ID generator - server requires this as additional state
#[cfg(feature = "server")]
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityIdGenerator {
    next_id: u64,
}
#[cfg(feature = "server")]
impl EntityIdGenerator {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }
}
#[cfg(feature = "server")]
impl Iterator for EntityIdGenerator {
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_id != 0 {
            let generated = self.next_id.into();
            self.next_id = self.next_id.wrapping_add(1);
            Some(generated)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
