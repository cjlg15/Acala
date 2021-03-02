
//! Autogenerated weights for module_cdp_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-26, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_cdp_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/cdp-treasury/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_cdp_treasury.
pub trait WeightInfo {
	fn auction_surplus() -> Weight;
	fn auction_debit() -> Weight;
	fn auction_collateral() -> Weight;
	fn set_collateral_auction_maximum_size() -> Weight;
}

/// Weights for module_cdp_treasury using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn auction_surplus() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn auction_debit() -> Weight {
		(26_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn auction_collateral() -> Weight {
		(2_124_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(204 as Weight))
	}
	fn set_collateral_auction_maximum_size() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn auction_surplus() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn auction_debit() -> Weight {
		(26_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn auction_collateral() -> Weight {
		(2_124_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(204 as Weight))
	}
	fn set_collateral_auction_maximum_size() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
