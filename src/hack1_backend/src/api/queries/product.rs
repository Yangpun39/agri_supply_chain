use crate::with_read_state;

// queary function to retrive product data.
#[ic_cdk::query]
pub fn api_get_my_product(product_id: String) -> Result<crate::models::product::Product, String> {
    with_read_state(|state| match state.product.get(&product_id) {
        Some(acc) => Ok(acc),
        None => Err(String::from(
            crate::utils::constants::ERROR_PRODUCT_NOT_REGISTERED,
        )),
    })
}

// function to get all the products specfic of the retailer.
#[ic_cdk::query]
pub fn api_get_all_product() -> Result<Vec<crate::models::product::Product>, String> {
    with_read_state(|state| {
        let products: Vec<_> = state.product.iter().map(|(_, product)| product.clone()).collect();
        if products.is_empty() {
            Err(String::from(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED))
        } else {
            Ok(products)
        }
    })
}
