//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-14, STEPS: [3, ], REPEAT: 2, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB
//! CACHE: 128

// Executed Command:
// ./target/release/cord
// benchmark
// --chain=dev
// --execution=wasm
// --pallet=pallet_scheduler
// --extrinsic=*
// --steps=3
// --repeat=2
// --output=./runtime/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_scheduler.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	fn schedule(_s: u32) -> Weight {
		(314_155_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel(s: u32) -> Weight {
		(501_890_000 as Weight)
			// Standard Error: 9_099_000
			.saturating_add((162_518_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn schedule_named(s: u32) -> Weight {
		(180_257_000 as Weight)
			// Standard Error: 415_000
			.saturating_add((100_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn cancel_named(s: u32) -> Weight {
		(143_070_000 as Weight)
			// Standard Error: 13_493_000
			.saturating_add((176_710_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
