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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Phase {
    Economic,
    Ordnance,
    Combat,
    Movement,
}

/// The state of the game
#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub major_bodies: HashMap<EntityId, MajorBody>,
    pub minor_bodies: HashMap<EntityId, MinorBody>,
    pub stacks: HashMap<EntityId, Stack>,
    pub warheads: HashMap<EntityId, Warhead>,
    pub phase: Phase,
}
impl GameState {
    #[cfg(feature = "server")]
    pub fn new(seed: <ChaCha20Rng as SeedableRng>::Seed, num_players: u8) -> GameState {
        todo!()
    }
}

/// A player ID
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
