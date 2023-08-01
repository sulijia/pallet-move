
//! Autogenerated weights for pallet_move
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Optimus-ROG`, CPU: `12th Gen Intel(R) Core(TM) i9-12950HX`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-move
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --output
// ../pallet-move/src/weights.rs
// --template
// ./.maintain/frame-weight-template.hbs
// --extrinsic
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_move.
pub trait WeightInfo {
	fn execute() -> Weight;
	fn publish_module() -> Weight;
	fn publish_package() -> Weight;
}

/// Weights for pallet_move using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_125_000 picoseconds.
		Weight::from_parts(5_568_000, 0)
	}
	fn publish_module() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_284_000 picoseconds.
		Weight::from_parts(5_814_000, 0)
	}
	fn publish_package() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_336_000 picoseconds.
		Weight::from_parts(5_700_000, 0)
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_125_000 picoseconds.
		Weight::from_parts(5_568_000, 0)
	}
	fn publish_module() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_284_000 picoseconds.
		Weight::from_parts(5_814_000, 0)
	}
	fn publish_package() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_336_000 picoseconds.
		Weight::from_parts(5_700_000, 0)
	}
}
