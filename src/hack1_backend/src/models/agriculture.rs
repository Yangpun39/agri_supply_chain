use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct Agriculture {
    pub data: String,           
    pub signing_key:  String,   
    pub verifying_key: String,
}

impl Storable for Agriculture {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: crate::utils::constants::STORABLE_USER_MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
