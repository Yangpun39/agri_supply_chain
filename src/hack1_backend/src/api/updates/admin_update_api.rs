use crate::utils::guards::*;


//customer account creation
#[ic_cdk::update(guard=guard_prevent_user_recreation)]
pub fn api_create_account(args: crate::models::agriculture::Agriculture) -> Result<String, String> {
    super::admin_controller::controller_create_account(args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::ERROR_ACCOUNT_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::SUCCESS_ACCOUNT_CREATED,
    ))
}
// //customer account updation
#[ic_cdk::update(guard=guard_check_admin)]
pub fn api_update_account(args: crate::models::agriculture::Agriculture) -> Result<String, String> {
    super::admin_controller::controller_update_account(args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::ERROR_ACCOUNT_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::SUCCESS_ACCOUNT_UPDATED,
    ))
}
#[ic_cdk::update(guard=guard_check_admin)]
pub fn api_delete_account(args:candid::Principal) -> Result<String, String> {
    super::admin_controller::controller_delete_account(args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::ERROR_ACCOUNT_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::SUCCESS_ACCOUNT_UPDATED,
    ))
}

// #[ic_cdk::update]
// pub fn api_subscription(key:String,value: String) -> Result<String, String> {
//     super::accounts_controller::controller_subscribe_sku(key,value).map_err(|err| {
//         format!(
//             "{}{}",
//             crate::utils::constants::ERROR_ACCOUNT_ERROR,
//             err.to_string()
//         )
//     })?;

//     Ok(String::from(
//         crate::utils::constants::SUBSCRIPTION_SUCCESSFUL,
//     ))
// }
