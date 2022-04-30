// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]

pub mod account;
pub mod algorithms;

pub mod devnet;
pub use devnet::*;

pub mod traits;
pub use traits::*;

use snarkvm_circuits_types::{environment::Environment, Boolean, Field, Group, Scalar};

pub trait Aleo: Environment {
    /// Returns the scalar multiplication on the group bases.
    fn g_scalar_multiply(scalar: &Scalar<Self>) -> Group<Self>;

    /// Returns a hash on the scalar field for the given input.
    fn hash_to_scalar(input: &[Field<Self>], rate: usize) -> Scalar<Self>;

    /// Returns a hash on the base field for the given input.
    fn hash_to_field(selector: &str, input: &[Boolean<Self>]) -> Field<Self>;

    /// Returns a commitment for the given input and randomness.
    fn commit(selector: &str, input: &[Boolean<Self>], randomness: &[Boolean<Self>]) -> Group<Self>;
}
