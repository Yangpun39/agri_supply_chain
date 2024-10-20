use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct Customer {
    pub data:String,
    pub signature: String,
    pub product_id:Option<Vec <String>>,
    pub permission:Option<Vec<candid::Principal>>,
}

impl Storable for Customer {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: crate::utils::constants::STORABLE_USER_MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
