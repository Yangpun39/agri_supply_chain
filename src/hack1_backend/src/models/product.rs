use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct Product {
    pub name: String,                       // Changed product_name to name
    pub category_id: u64,                   // Changed category to category_id
    pub brand: String,
    pub umbrella_brand: String,             // Changed umbrella_brand_name to umbrella_brand
    pub regular_price: f64,                 // Changed type to f64 for numeric price
    pub stock_level: u64,                   // Changed type to u64 for numeric stock level
    pub supplier: String,                   // Changed supplier_name to supplier
    pub description: String,                // Changed product_desc to description
    pub start_date: u64,                    // Changed start_data to start_date and type to u64 for timestamp
    pub end_date: u64,                      // Changed end_data to end_date and type to u64 for timestamp
    pub original_price: f64,                // Changed type to f64 for numeric price
    pub price_after_promotion: f64,         // Changed type to f64 for numeric price
    pub promotion_description: Option<String>, // Changed promotion_desc to promotion_description
}

impl Storable for Product {
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
