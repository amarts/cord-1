// CORD Blockchain – https://dhiway.network
// Copyright (C) 2019-2021 Dhiway
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Low-level types used throughout the CORD code.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::Weight;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature, OpaqueExtrinsic,
};
use sp_std::vec::Vec;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on
/// the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it
/// equivalent to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of
/// them.
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

// A timestamp: milliseconds since the unix epoch.
/// `u64` is enough to represent a duration of half a billion years, when the
/// time scale is milliseconds.
pub type Timestamp = u64;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;
/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;
/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// CID type.
pub type IdentifierOf = Vec<u8>;

/// status Information
pub type StatusOf = bool;

/// App-specific crypto used for reporting equivocation/misbehavior in BABE and
/// GRANDPA. Any rewards for misbehavior reporting will be paid out to this
/// account.
pub mod report {
	use super::{Signature, Verify};
	use frame_system::offchain::AppCrypto;
	use sp_core::crypto::{key_types, KeyTypeId};

	/// Key type for the reporting module. Used for reporting BABE and GRANDPA
	/// equivocations.
	pub const KEY_TYPE: KeyTypeId = key_types::REPORTING;

	mod app {
		use sp_application_crypto::{app_crypto, sr25519};
		app_crypto!(sr25519, super::KEY_TYPE);
	}

	/// Identity of the equivocation/misbehavior reporter.
	pub type ReporterId = app::Public;

	/// An `AppCrypto` type to allow submitting signed transactions using the
	/// reporting application key as signer.
	pub struct ReporterAppCrypto;

	impl AppCrypto<<Signature as Verify>::Signer, Signature> for ReporterAppCrypto {
		type RuntimeAppPublic = ReporterId;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}
/// A trait that allows version migrators to access the underlying pallet's
/// context, e.g., its Config trait.
///
/// In this way, the migrator can access the pallet's storage and the pallet's
/// types directly.
pub trait VersionMigratorTrait<T> {
	#[cfg(feature = "try-runtime")]
	fn pre_migrate(&self) -> Result<(), &str>;
	fn migrate(&self) -> Weight;
	#[cfg(feature = "try-runtime")]
	fn post_migrate(&self) -> Result<(), &str>;
}
