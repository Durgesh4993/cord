// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_did::WeightInfo for WeightInfo<T> {
   // Storage: System Account (r:2 w:2)
	// Storage: Did DidBlacklist (r:1 w:0)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:25)
	/// The range of component `n` is `[1, 10]`.
	/// The range of component `c` is `[1, 25]`.
	fn create_ed25519_keys(n: u32, c: u32, ) -> Weight {
		Weight::from_parts(155_362_954,0)
			// Standard Error: 11_821
			.saturating_add(Weight::from_parts(1_263_549,0).saturating_mul(n as u64))
			// Standard Error: 11_821
			.saturating_add(Weight::from_parts(1_263_549,0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c.into())))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Did DidBlacklist (r:1 w:0)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:25)
	/// The range of component `n` is `[1, 10]`.
	/// The range of component `c` is `[1, 25]`.
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		Weight::from_parts(155_463_794,0)
			// Standard Error: 12_431
			.saturating_add(Weight::from_parts(1_422_221,0).saturating_mul(n as u64))
			// Standard Error: 12_431
			.saturating_add(Weight::from_parts(1_422_221,0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c.into())))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Did DidBlacklist (r:1 w:0)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:25)
	/// The range of component `n` is `[1, 10]`.
	/// The range of component `c` is `[1, 25]`.
	fn create_ecdsa_keys(n: u32, c: u32 ) -> Weight {
		Weight::from_parts(141_772_067,0)
			// Standard Error: 10_731
			.saturating_add(Weight::from_parts(1_239_702,0).saturating_mul(n as u64))
			// Standard Error: 10_731
			.saturating_add(Weight::from_parts(1_239_702,0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c.into())))
	}
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidBlacklist (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:1)
	/// The range of component `c` is `[1, 25]`.
	fn delete(c: u32, ) -> Weight {
		Weight::from_parts(45_992_948,0)
			// Standard Error: 6_651
			.saturating_add(Weight::from_parts(1_286_699,0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ed25519_key() -> Weight {
		Weight::from_parts(94_077_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_sr25519_key() -> Weight {
		Weight::from_parts(95_122_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ecdsa_key() -> Weight {
		Weight::from_parts(80_276_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_authentication_key() -> Weight {
		Weight::from_parts(44_189_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_authentication_key() -> Weight {
		Weight::from_parts(43_575_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_authentication_key() -> Weight {
		Weight::from_parts(43_395_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_delegation_key() -> Weight {
		Weight::from_parts(43_007_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_delegation_key() -> Weight {
		Weight::from_parts(43_625_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_delegation_key() -> Weight {
		Weight::from_parts(43_135_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_delegation_key() -> Weight {
		Weight::from_parts(40_555_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_delegation_key() -> Weight {
		Weight::from_parts(40_539_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_delegation_key() -> Weight {
		Weight::from_parts(40_439_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_assertion_key() -> Weight {
		Weight::from_parts(43_628_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_assertion_key() -> Weight {
		Weight::from_parts(43_908_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_assertion_key() -> Weight {
		Weight::from_parts(43_418_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_assertion_key() -> Weight {
		Weight::from_parts(40_519_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_assertion_key() -> Weight {
		Weight::from_parts(40_720_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_assertion_key() -> Weight {
		Weight::from_parts(40_956_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_ed25519_key_agreement_key() -> Weight {
		Weight::from_parts(42_934_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_sr25519_key_agreement_key() -> Weight {
		Weight::from_parts(42_870_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_ecdsa_key_agreement_key() -> Weight {
		Weight::from_parts(42_610_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_key_agreement_key() -> Weight {
		Weight::from_parts(40_817_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_key_agreement_key() -> Weight {
		Weight::from_parts(41_022_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_key_agreement_key() -> Weight {
		Weight::from_parts(40_682_000,0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did ServiceEndpoints (r:1 w:1)
	fn add_service_endpoint() -> Weight {
		Weight::from_parts(51_035_000,0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Did ServiceEndpoints (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	fn remove_service_endpoint() -> Weight {
		Weight::from_parts(42_794_000,0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	/// The range of component `l` is `[1, 5242880]`.
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		Weight::from_parts(42_471_912,0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(4_136,0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	/// The range of component `l` is `[1, 5242880]`.
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		Weight::from_parts(44_021_425,0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(2_466,0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	/// The range of component `l` is `[1, 5242880]`.
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		Weight::from_parts(25_362_991,0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_491,0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
}
