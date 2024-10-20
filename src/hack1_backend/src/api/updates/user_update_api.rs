use crate::utils::guards::*;


//account creation
#[ic_cdk::update(guard=guard_prevent_user_recreation)]
pub fn api_create_farmer( args: String,signing_key:String) -> Result<String, String> {
    super::user_controller::controller_create_farmer(args,signing_key).map_err(|err| {
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

//retailer update
#[ic_cdk::update(guard=guard_prevent_user_recreation)]
pub fn api_update_farmer(args: crate::models::farmer::Farmer) -> Result<String, String> {
    super::user_controller::controller_update_farmer(args).map_err(|err| {
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
//account creation
#[ic_cdk::update(guard=guard_prevent_user_recreation)]
pub fn api_create_distributor( args: String,signing_key:String) -> Result<String, String> {
    super::user_controller::controller_create_distributor(args,signing_key).map_err(|err| {
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

//retailer update
#[ic_cdk::update(guard=guard_prevent_user_recreation)]
pub fn api_update_distributor(args: crate::models::distributor::Distributor) -> Result<String, String> {
    super::user_controller::controller_update_distributor(args).map_err(|err| {
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
//account creation
#[ic_cdk::update(guard=guard_prevent_user_recreation)]
pub fn api_create_retailer( args: String,signing_key:String) -> Result<String, String> {
    super::user_controller::controller_create_retailer(args,signing_key).map_err(|err| {
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

//retailer update
#[ic_cdk::update(guard=guard_prevent_user_recreation)]
pub fn api_update_retailer(args: crate::models::retailer_types::RetailerProfile) -> Result<String, String> {
    super::user_controller::controller_update_retailer(args).map_err(|err| {
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


// //adding store detail
// #[ic_cdk::update(guard=guard_prevent_anonymous_retailer)]
// pub fn api_add_retailer_store(key:String,args:crate::models::store_detail::StoreDetail)-> Result<String, String> {
//     super::retailer_controller::controller_store_retailer(&key,args).map_err(|err| {
//         format!(
//             "{}{}",
//             crate::utils::constants::ERROR_STORE_ERROR,
//             err.to_string()
//         )
//     })?;

//     Ok(String::from(
//         crate::utils::constants::SUCCESS_STORE_CREATED,
//     ))
// }

// //upgrading store detail
// #[ic_cdk::update(guard=guard_prevent_anonymous_retailer)]
// pub fn api_update_retailer_store(key:String,args:crate::models::store_detail::StoreDetail)-> Result<String, String> {
//     super::retailer_controller::controller_store_retailer_update(key.clone(),args).map_err(|err| {
//         format!(
//             "{}{}",
//             crate::utils::constants::ERROR_STORE_ERROR,
//             err.to_string()
//         )
//     })?;

//     Ok(String::from(
//         crate::utils::constants::SUCCESS_STORE_UPDATED,
//     ))
// }