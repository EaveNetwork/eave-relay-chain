// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use sp_std::prelude::*;
use xcm::v0::{MultiOrigin, MultiLocation, SendXcm, XcmError};
use sp_runtime::traits::Convert;
use frame_support::dispatch::{Dispatchable, Parameter};
use crate::traits::{TransactAsset, ConvertOrigin};

pub trait Config {
	/// The outer call dispatch type.
	type Call: Parameter + Dispatchable;

	type XcmSender: SendXcm;

	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// How to get a call origin from a `MultiOrigin` value.
	type OriginConverter: ConvertOrigin<<Self::Call as Dispatchable>::Origin>;
}
