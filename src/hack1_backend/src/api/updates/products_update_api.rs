// // main function from where we are calling the controller product create function.
// #[ic_cdk::update]
// pub fn api_create_product(key: String,args: crate::models::product::Product) -> Result<String, String> {
//     super::products_controller::controller_create_product(key.clone(),args).map_err(|err| {
//         format!(
//             "{}{}",
//             crate::utils::constants::ERROR_PRODUCT_ERROR,
//             err.to_string()
//         )
//     })?;

//     Ok(String::from(crate::utils::constants::SUCCESS_PRODUCT_CREATED))
// }

// // function to update the product.
// #[ic_cdk::update]
// pub fn api_update_product(key:String,args: crate::models::product::Product) -> Result<String, String> {
//     super::products_controller::controller_update_product(key.clone(),args).map_err(|err| {
//         format!(
//             "{}{}",
//             crate::utils::constants::ERROR_PRODUCT_ERROR,
//             err.to_string()
//         )
//     })?;

//     Ok(String::from(
//         crate::utils::constants::SUCCESS_PRODUCT_UPDATED,
//     ))
// }
// //adding promotion
#[ic_cdk::update]
pub fn api_add_farmer_product(key:String,args:crate::models::product::Product)-> Result<String, String> {
    super::products_controller::controller_product_farmer(&key,args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::PROMO_ADD_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::PROMO_ADD_SUCCESS,
    ))
}

#[ic_cdk::update]
pub fn api_add_distributor_product(key:String,args:crate::models::product::Product)-> Result<String, String> {
    super::products_controller::controller_product_distributor(&key,args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::PROMO_ADD_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::PROMO_ADD_SUCCESS,
    ))
}

#[ic_cdk::update]
pub fn api_add_retailer_product(key:String,args:crate::models::product::Product)-> Result<String, String> {
    super::products_controller::controller_product_retailer(&key,args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::PROMO_ADD_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::PROMO_ADD_SUCCESS,
    ))
}
#[ic_cdk::update]
pub fn api_update_product(key:String,args: crate::models::product::Product) -> Result<String, String> {
    super::products_controller::controller_update_product(key.clone(),args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::ERROR_PROMOTION_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::SUCCESS_PROMOTION_UPDATED,
    ))
}
 

#[ic_cdk::update]
pub fn api_delete_product(key:String) -> Result<String, String> {
    super::products_controller::controller_delete_product(key.clone()).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::ERROR_PROMOTION_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::SUCCESS_PROMOTION_UPDATED,
    ))
}