// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use solana_client::rpc_response::SlotInfo as SdkSlotInfo;

#[derive(Clone, Copy, Debug)]
pub struct SlotInfo {
    pub slot: Slot,
    pub parent: Slot,
    pub root: Slot,
}

impl From<SdkSlotInfo> for SlotInfo {
    fn from(value: SdkSlotInfo) -> Self {
        SlotInfo {
            slot: value.slot.into(),
            parent: value.parent.into(),
            root: value.parent.into(),
        }
    }
}

// #[derive(Clone, Debug)]
// pub struct KeyedAccount {
//     pub slot: Slot,
//     pub pubkey: PublicKey,
//     pub account: AccountInfo,
// }
