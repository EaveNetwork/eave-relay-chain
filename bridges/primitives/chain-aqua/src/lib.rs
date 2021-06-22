// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
// RuntimeApi generated functions
#![allow(clippy::too_many_arguments)]
// Runtime-generated DecodeLimit::decode_all_with_depth_limit
#![allow(clippy::unnecessary_mut_passed)]

use bp_messages::{LaneId, MessageNonce, UnrewardedRelayersState, Weight};
use bp_runtime::Chain;
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub use bp_polkadot_core::*;

/// Aqua Chain
pub type Aqua = PolkadotLike;

pub type UncheckedExtrinsic = bp_polkadot_core::UncheckedExtrinsic<Call>;

// NOTE: This needs to be kept up to date with the Aqua runtime found in the Polkadot repo.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: sp_version::create_runtime_str!("aqua"),
	impl_name: sp_version::create_runtime_str!("parity-aqua-v1.0"),
	authoring_version: 0,
	spec_version: 232,
	impl_version: 0,
	apis: sp_version::create_apis_vec![[]],
	transaction_version: 0,
};

/// Aqua Runtime `Call` enum.
///
/// The enum represents a subset of possible `Call`s we can send to Aqua chain.
/// Ideally this code would be auto-generated from Metadata, because we want to
/// avoid depending directly on the ENTIRE runtime just to get the encoding of `Dispatchable`s.
///
/// All entries here (like pretty much in the entire file) must be kept in sync with Aqua
/// `construct_runtime`, so that we maintain SCALE-compatibility.
///
/// See: https://github.com/paritytech/polkadot/blob/master/runtime/aqua/src/lib.rs
#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq, Clone)]
pub enum Call {
	/// Wococo bridge pallet.
	#[codec(index = 41)]
	BridgeGrandpaWococo(BridgeGrandpaWococoCall),
}

#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum BridgeGrandpaWococoCall {
	#[codec(index = 0)]
	submit_finality_proof(
		<PolkadotLike as Chain>::Header,
		bp_header_chain::justification::GrandpaJustification<<PolkadotLike as Chain>::Header>,
	),
	#[codec(index = 1)]
	initialize(bp_header_chain::InitializationData<<PolkadotLike as Chain>::Header>),
}

impl sp_runtime::traits::Dispatchable for Call {
	type Origin = ();
	type Config = ();
	type Info = ();
	type PostInfo = ();

	fn dispatch(self, _origin: Self::Origin) -> sp_runtime::DispatchResultWithInfo<Self::PostInfo> {
		unimplemented!("The Call is not expected to be dispatched.")
	}
}

/// Name of the `AquaFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_AQUA_HEADER_METHOD: &str = "AquaFinalityApi_best_finalized";
/// Name of the `AquaFinalityApi::is_known_header` runtime method.
pub const IS_KNOWN_AQUA_HEADER_METHOD: &str = "AquaFinalityApi_is_known_header";

/// Name of the `ToAquaOutboundLaneApi::estimate_message_delivery_and_dispatch_fee` runtime method.
pub const TO_AQUA_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"ToAquaOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `ToAquaOutboundLaneApi::messages_dispatch_weight` runtime method.
pub const TO_AQUA_MESSAGES_DISPATCH_WEIGHT_METHOD: &str = "ToAquaOutboundLaneApi_messages_dispatch_weight";
/// Name of the `ToAquaOutboundLaneApi::latest_generated_nonce` runtime method.
pub const TO_AQUA_LATEST_GENERATED_NONCE_METHOD: &str = "ToAquaOutboundLaneApi_latest_generated_nonce";
/// Name of the `ToAquaOutboundLaneApi::latest_received_nonce` runtime method.
pub const TO_AQUA_LATEST_RECEIVED_NONCE_METHOD: &str = "ToAquaOutboundLaneApi_latest_received_nonce";

/// Name of the `FromAquaInboundLaneApi::latest_received_nonce` runtime method.
pub const FROM_AQUA_LATEST_RECEIVED_NONCE_METHOD: &str = "FromAquaInboundLaneApi_latest_received_nonce";
/// Name of the `FromAquaInboundLaneApi::latest_onfirmed_nonce` runtime method.
pub const FROM_AQUA_LATEST_CONFIRMED_NONCE_METHOD: &str = "FromAquaInboundLaneApi_latest_confirmed_nonce";
/// Name of the `FromAquaInboundLaneApi::unrewarded_relayers_state` runtime method.
pub const FROM_AQUA_UNREWARDED_RELAYERS_STATE: &str = "FromAquaInboundLaneApi_unrewarded_relayers_state";

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized Aqua headers.
	///
	/// This API is implemented by runtimes that are bridging with the Aqua chain, not the
	/// Aqua runtime itself.
	pub trait AquaFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> (BlockNumber, Hash);
		/// Returns true if the header is known to the runtime.
		fn is_known_header(hash: Hash) -> bool;
	}

	/// Outbound message lane API for messages that are sent to Aqua chain.
	///
	/// This API is implemented by runtimes that are sending messages to Aqua chain, not the
	/// Aqua runtime itself.
	pub trait ToAquaOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to Aqua from this chain.
		///
		/// Please keep in mind that this method returns the lowest message fee required for message
		/// to be accepted to the lane. It may be good idea to pay a bit over this price to account
		/// future exchange rate changes and guarantee that relayer would deliver your message
		/// to the target chain.
		fn estimate_message_delivery_and_dispatch_fee(
			lane_id: LaneId,
			payload: OutboundPayload,
		) -> Option<OutboundMessageFee>;
		/// Returns total dispatch weight and encoded payload size of all messages in given inclusive range.
		///
		/// If some (or all) messages are missing from the storage, they'll also will
		/// be missing from the resulting vector. The vector is ordered by the nonce.
		fn messages_dispatch_weight(
			lane: LaneId,
			begin: MessageNonce,
			end: MessageNonce,
		) -> Vec<(MessageNonce, Weight, u32)>;
		/// Returns nonce of the latest message, received by bridged chain.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Returns nonce of the latest message, generated by given lane.
		fn latest_generated_nonce(lane: LaneId) -> MessageNonce;
	}

	/// Inbound message lane API for messages sent by Aqua chain.
	///
	/// This API is implemented by runtimes that are receiving messages from Aqua chain, not the
	/// Aqua runtime itself.
	pub trait FromAquaInboundLaneApi {
		/// Returns nonce of the latest message, received by given lane.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Nonce of the latest message that has been confirmed to the bridged chain.
		fn latest_confirmed_nonce(lane: LaneId) -> MessageNonce;
		/// State of the unrewarded relayers set at given lane.
		fn unrewarded_relayers_state(lane: LaneId) -> UnrewardedRelayersState;
	}
}
