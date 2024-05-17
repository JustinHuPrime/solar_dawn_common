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

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use serde::{Deserialize, Serialize};

/// An axial, hex-grid, point-up 2-d vector position
///
/// Increasing q = up-left
/// Increasing r = down
#[cfg_attr(
    any(feature = "client", feature = "server"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub q: i64,
    pub r: i64,
}
impl Position {
    pub fn new(q: i64, r: i64) -> Self {
        Self { q, r }
    }
}
impl AddAssign<Displacement> for Position {
    fn add_assign(&mut self, rhs: Displacement) {
        self.q += rhs.q;
        self.r += rhs.r;
    }
}
impl Add<Displacement> for Position {
    type Output = Position;

    fn add(self, rhs: Displacement) -> Self::Output {
        let mut value = self;
        value += rhs;
        value
    }
}
impl SubAssign<Displacement> for Position {
    fn sub_assign(&mut self, rhs: Displacement) {
        self.q -= rhs.q;
        self.r -= rhs.r;
    }
}
impl Sub<Displacement> for Position {
    type Output = Position;

    fn sub(self, rhs: Displacement) -> Self::Output {
        let mut value = self;
        value -= rhs;
        value
    }
}

/// An axial, hex-grid, point-up 2-d vector displacement
///
/// Increasing q = up-left
/// Increasing r = down
#[cfg_attr(
    any(feature = "client", feature = "server"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Displacement {
    pub q: i64,
    pub r: i64,
}
impl Displacement {
    pub fn new(q: i64, r: i64) -> Self {
        Self { q, r }
    }
}
impl Neg for Displacement {
    type Output = Displacement;

    fn neg(self) -> Self::Output {
        Self::Output {
            q: -self.q,
            r: -self.r,
        }
    }
}
impl AddAssign<Displacement> for Displacement {
    fn add_assign(&mut self, rhs: Displacement) {
        self.q += rhs.q;
        self.r += rhs.r;
    }
}
impl Add<Displacement> for Displacement {
    type Output = Displacement;

    fn add(self, rhs: Displacement) -> Self::Output {
        let mut value = self;
        value += rhs;
        value
    }
}
impl Add<Position> for Displacement {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        rhs + self
    }
}
impl SubAssign<Displacement> for Displacement {
    fn sub_assign(&mut self, rhs: Displacement) {
        self.q -= rhs.q;
        self.r -= rhs.r;
    }
}
impl Sub<Displacement> for Displacement {
    type Output = Displacement;

    fn sub(self, rhs: Displacement) -> Self::Output {
        let mut value = self;
        value -= rhs;
        value
    }
}
impl MulAssign<i64> for Displacement {
    fn mul_assign(&mut self, rhs: i64) {
        self.q *= rhs;
        self.r *= rhs;
    }
}
impl Mul<i64> for Displacement {
    type Output = Displacement;

    fn mul(self, rhs: i64) -> Self::Output {
        let mut value = self;
        value *= rhs;
        value
    }
}
impl Mul<Displacement> for i64 {
    type Output = Displacement;

    fn mul(self, rhs: Displacement) -> Self::Output {
        let mut value = rhs;
        value *= self;
        value
    }
}
impl DivAssign<i64> for Displacement {
    fn div_assign(&mut self, rhs: i64) {
        self.q /= rhs;
        self.r /= rhs;
    }
}
impl Div<i64> for Displacement {
    type Output = Displacement;

    fn div(self, rhs: i64) -> Self::Output {
        let mut value = self;
        value /= rhs;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_construction() {
        let value = Position::new(5, 6);
        assert_eq!(value.q, 5);
        assert_eq!(value.r, 6);
    }

    #[test]
    fn test_position_ops() {
        let pos = Position::new(5, 6);
        let displacement = Displacement::new(1, 3);
        let result = pos + displacement;
        assert_eq!(result.q, pos.q + displacement.q);
        assert_eq!(result.r, pos.r + displacement.r);

        let mut pos = Position::new(5, 6);
        let displacement = Displacement::new(1, 3);
        pos += displacement;
        assert_eq!(pos.q, 5 + displacement.q);
        assert_eq!(pos.r, 6 + displacement.r);

        let pos = Position::new(5, 6);
        let displacement = Displacement::new(1, 3);
        let result = pos - displacement;
        assert_eq!(result.q, pos.q - displacement.q);
        assert_eq!(result.r, pos.r - displacement.r);

        let mut pos = Position::new(5, 6);
        let displacement = Displacement::new(1, 3);
        pos -= displacement;
        assert_eq!(pos.q, 5 - displacement.q);
        assert_eq!(pos.r, 6 - displacement.r);

        let a = Position::new(1, 2);
        let b = Position::new(1, 2);
        let c = Position::new(1, 3);
        let d = Position::new(2, 2);
        assert!(a == b);
        assert!(a != c);
        assert!(a != d);
    }

    #[test]
    fn test_displacement_construction() {
        let value = Displacement::new(5, 6);
        assert_eq!(value.q, 5);
        assert_eq!(value.r, 6);
    }

    #[test]
    fn test_displacement_ops() {
        let a = Displacement::new(2, 3);
        let result = -a;
        assert_eq!(result.q, -2);
        assert_eq!(result.r, -3);

        let mut a = Displacement::new(2, 3);
        let b = Displacement::new(4, 6);
        a += b;
        assert_eq!(a.q, 2 + b.q);
        assert_eq!(a.r, 3 + b.r);

        let a = Displacement::new(2, 3);
        let b = Displacement::new(4, 6);
        let result = a + b;
        assert_eq!(result.q, a.q + b.q);
        assert_eq!(result.r, a.r + b.r);

        let a = Displacement::new(2, 3);
        let b = Position::new(4, 6);
        let result = a + b;
        assert_eq!(result.q, a.q + b.q);
        assert_eq!(result.r, a.r + b.r);

        let mut a = Displacement::new(2, 3);
        let b = Displacement::new(4, 6);
        a -= b;
        assert_eq!(a.q, 2 - b.q);
        assert_eq!(a.r, 3 - b.r);

        let a = Displacement::new(2, 3);
        let b = Displacement::new(4, 6);
        let result = a - b;
        assert_eq!(result.q, a.q - b.q);
        assert_eq!(result.r, a.r - b.r);

        let mut a = Displacement::new(2, 3);
        a *= 2;
        assert_eq!(a.q, 2 * 2);
        assert_eq!(a.r, 3 * 2);

        let a = Displacement::new(2, 3);
        let result = a * 2;
        assert_eq!(result.q, a.q * 2);
        assert_eq!(result.r, a.r * 2);

        let a = Displacement::new(2, 3);
        let result = 2 * a;
        assert_eq!(result.q, a.q * 2);
        assert_eq!(result.r, a.r * 2);

        let mut a = Displacement::new(2, 5);
        a /= 2;
        assert_eq!(a.q, 2 / 2);
        assert_eq!(a.r, 5 / 2);

        let a = Displacement::new(2, 5);
        let result = a / 2;
        assert_eq!(result.q, a.q / 2);
        assert_eq!(result.r, a.r / 2);

        let a = Displacement::new(1, 2);
        let b = Displacement::new(1, 2);
        let c = Displacement::new(1, 3);
        let d = Displacement::new(2, 2);
        assert!(a == b);
        assert!(a != c);
        assert!(a != d);
    }
}
