use crate::with_read_state;

#[ic_cdk::query]
pub fn api_get_my_farmer() -> Result<crate::models::farmer::Farmer, String> {
    with_read_state(|state| match state.farmer.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(acc),
        None => Err(String::from(
            crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}
#[ic_cdk::query]
pub fn api_get_my_distributor() -> Result<crate::models::distributor::Distributor, String> {
    with_read_state(|state| match state.distributor.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(acc),
        None => Err(String::from(
            crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}
#[ic_cdk::query]
pub fn api_get_my_retailer() -> Result<crate::models::retailer_types::RetailerProfile, String> {
    with_read_state(|state| match state.retailer.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(acc),
        None => Err(String::from(
            crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}
#[ic_cdk::query]
pub fn api_get_product_farmer() -> Result<Vec<crate::models::product::Product>, String> {
    with_read_state(|state| {
        if let Some(retailer_profile) = state.farmer.get(&ic_cdk::api::caller()) {
            if let Some(promo_ids) = &retailer_profile.product_id {
                let mut promotions = Vec::new();
                for promo_id in promo_ids {
                    if let Some(promo) = state.product.get(promo_id) {
                        promotions.push(promo.clone());
                    } else {
                        return Err(format!("Promotion ID {} not found", promo_id));
                    }
                }
                Ok(promotions)
            } else {
                Ok(Vec::new())
            }
        } else {
            Err(String::from(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED))
        }
    })
}

#[ic_cdk::query]
pub fn api_get_product_distributor() -> Result<Vec<crate::models::product::Product>, String> {
    with_read_state(|state| {
        if let Some(retailer_profile) = state.distributor.get(&ic_cdk::api::caller()) {
            if let Some(promo_ids) = &retailer_profile.product_id {
                let mut promotions = Vec::new();
                for promo_id in promo_ids {
                    if let Some(promo) = state.product.get(promo_id) {
                        promotions.push(promo.clone());
                    } else {
                        return Err(format!("Promotion ID {} not found", promo_id));
                    }
                }
                Ok(promotions)
            } else {
                Ok(Vec::new())
            }
        } else {
            Err(String::from(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED))
        }
    })
}

#[ic_cdk::query]
pub fn api_get_product_retailer() -> Result<Vec<crate::models::product::Product>, String> {
    with_read_state(|state| {
        if let Some(retailer_profile) = state.retailer.get(&ic_cdk::api::caller()) {
            if let Some(promo_ids) = &retailer_profile.product_id {
                let mut promotions = Vec::new();
                for promo_id in promo_ids {
                    if let Some(promo) = state.product.get(promo_id) {
                        promotions.push(promo.clone());
                    } else {
                        return Err(format!("Promotion ID {} not found", promo_id));
                    }
                }
                Ok(promotions)
            } else {
                Ok(Vec::new())
            }
        } else {
            Err(String::from(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED))
        }
    })
}

