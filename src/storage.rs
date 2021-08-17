use crate::types::{ChannelId, Dispute, FundingId, NativeBalance};
use cw_storage_plus::Map;

pub const DEPOSITS: Map<FundingId, NativeBalance> = Map::new("deposits");
pub const DISPUTES: Map<ChannelId, Dispute> = Map::new("register");
