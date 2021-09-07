//  Copyright 2021 PolyCrypt GmbH
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

//! Definition of the on-chain storage containers.
use crate::types::{ChannelId, Dispute, FundingId, WrappedNativeBalance};
use cw_storage_plus::Map;

pub const DEPOSITS: Map<FundingId, WrappedNativeBalance> = Map::new("deposits");
pub const DISPUTES: Map<ChannelId, Dispute> = Map::new("register");
