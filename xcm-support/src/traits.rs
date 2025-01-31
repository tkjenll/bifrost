// This file is part of Bifrost.

// Copyright (C) 2019-2021 Liebi Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

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

use node_primitives::MessageId;
use xcm::{latest::prelude::*, DoubleEncoded};

/// Bifrost Xcm Executor
pub trait BifrostXcmExecutor {
	fn transact_weight(weight: u64, nonce: u32) -> u64;

	fn transact_id(data: &[u8]) -> MessageId;

	fn ump_transact(
		origin: MultiLocation,
		call: DoubleEncoded<()>,
		weight: u64,
		relay: bool,
		nonce: u32,
	) -> Result<MessageId, XcmError>;

	fn ump_transfer_asset(
		origin: MultiLocation,
		dest: MultiLocation,
		amount: u128,
		relay: bool,
		nonce: u32,
	) -> Result<MessageId, XcmError>;
}
