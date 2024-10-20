use crate::utils::guards::*;
use crate::with_read_state;

#[ic_cdk::query(guard=guard_check_admin)]
pub fn api_get_my_account() -> Result<crate::models::agriculture::Agriculture, String> {
    with_read_state(|state| match state.agriculture_department.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(acc),
        None => Err(String::from(
            crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}
#[ic_cdk::query]
pub fn api_verification(
    id: candid::Principal, 
    verify_key: String
) -> Result<bool, String> {
    with_read_state(|state| {
        match state.user.get(&id) {
            Some(account_type) => match account_type.as_str() {
                "distributor" => {
                    if let Some(distributor) = state.distributor.get(&id) {
                        let result = crate::api::updates::digital_verify::verify_signature(
                            &verify_key,
                            distributor.data.as_bytes(),
                            &distributor.signature,
                        );
                        Ok(result)
                    } else {
                        Err("Distributor not found".into())
                    }
                }
                "retailer" => {
                    if let Some(retailer) = state.retailer.get(&id) {
                        let result = crate::api::updates::digital_verify::verify_signature(
                            &verify_key,
                            retailer.data.as_bytes(),
                            &retailer.signature,
                        );
                        Ok(result)
                    } else {
                        Err("Retailer not found".into())
                    }
                }
                "farmer" => {
                    if let Some(farmer) = state.farmer.get(&id) {
                        let result = crate::api::updates::digital_verify::verify_signature(
                            &verify_key,
                            farmer.data.as_bytes(),
                            &farmer.signature,
                        );
                        Ok(result)
                    } else {
                        Err("Farmer not found".into())
                    }
                }
                _ => Err("Unknown account type".into()),
            },
            None => Err(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED.into()),
        }
    })
}