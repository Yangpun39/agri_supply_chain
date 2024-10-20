use crate::with_write_state;
pub fn controller_product_farmer(
    key: &str,
    args: crate::models::product::Product,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(mut retailer_profile) = state.farmer.get(&ic_cdk::api::caller()) {
            if state.product.contains_key(&key.to_string()) {
                return Err(String::from(
                    crate::utils::constants::WARNING_PROMOTION_EXISTS,
                ));
            }
            state.product.insert(key.to_string(), args);
            if let Some(promo_ids) = &mut retailer_profile.product_id {
                promo_ids.push(key.to_string());
            } else {
                retailer_profile.product_id = Some(vec![key.to_string()]);
            }
            state
                .farmer
                .insert(ic_cdk::api::caller(), retailer_profile);
            Ok(())
        } else {
            Err(String::from(
                crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}

pub fn controller_product_distributor(
    key: &str,
    args: crate::models::product::Product,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(mut retailer_profile) = state.distributor.get(&ic_cdk::api::caller()) {
            if state.product.contains_key(&key.to_string()) {
                return Err(String::from(
                    crate::utils::constants::WARNING_PROMOTION_EXISTS,
                ));
            }
            state.product.insert(key.to_string(), args);
            if let Some(promo_ids) = &mut retailer_profile.product_id {
                promo_ids.push(key.to_string());
            } else {
                retailer_profile.product_id = Some(vec![key.to_string()]);
            }
            state
                .distributor
                .insert(ic_cdk::api::caller(), retailer_profile);
            Ok(())
        } else {
            Err(String::from(
                crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}

pub fn controller_product_retailer(
    key: &str,
    args: crate::models::product::Product,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(mut retailer_profile) = state.retailer.get(&ic_cdk::api::caller()) {
            if state.product.contains_key(&key.to_string()) {
                return Err(String::from(
                    crate::utils::constants::WARNING_PROMOTION_EXISTS,
                ));
            }
            state.product.insert(key.to_string(), args);
            if let Some(promo_ids) = &mut retailer_profile.product_id {
                promo_ids.push(key.to_string());
            } else {
                retailer_profile.product_id = Some(vec![key.to_string()]);
            }
            state
                .retailer
                .insert(ic_cdk::api::caller(), retailer_profile);
            Ok(())
        } else {
            Err(String::from(
                crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}

pub fn controller_update_product(
    key: String,
    args: crate::models::product::Product,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(existing_product) = state.product.get(&key) {
            let updated_promo = crate::models::product::Product {
                name: if args.name.is_empty() {
                    existing_product.name.clone()
                } else {
                    args.name.clone()
                },
                category_id: if args.category_id == 0 {
                    existing_product.category_id
                } else {
                    args.category_id
                },
                brand: if args.brand.is_empty() {
                    existing_product.brand.clone()
                } else {
                    args.brand.clone()
                },
                umbrella_brand: if args.umbrella_brand.is_empty() {
                    existing_product.umbrella_brand.clone()
                } else {
                    args.umbrella_brand.clone()
                },
                regular_price: if args.regular_price == 0.0 {
                    existing_product.regular_price
                } else {
                    args.regular_price
                },
                stock_level: if args.stock_level == 0 {
                    existing_product.stock_level
                } else {
                    args.stock_level
                },
                supplier: if args.supplier.is_empty() {
                    existing_product.supplier.clone()
                } else {
                    args.supplier.clone()
                },
                description: if args.description.is_empty() {
                    existing_product.description.clone()
                } else {
                    args.description.clone()
                },
                start_date: if args.start_date == 0 {
                    existing_product.start_date
                } else {
                    args.start_date
                },
                end_date: if args.end_date == 0 {
                    existing_product.end_date
                } else {
                    args.end_date
                },
                original_price: if args.original_price == 0.0 {
                    existing_product.original_price
                } else {
                    args.original_price
                },
                price_after_promotion: if args.price_after_promotion == 0.0 {
                    existing_product.price_after_promotion
                } else {
                    args.price_after_promotion
                },
                promotion_description: if args.promotion_description.is_none() {
                    existing_product.promotion_description.clone()
                } else {
                    args.promotion_description.clone()
                },
            };
            state.product.insert(key.clone(), updated_promo);
            Ok(())
        } else {
            Err(String::from(crate::utils::constants::ERROR_PROMO_NOT_FOUND))
        }
    })
}

pub fn controller_delete_product(key: String) -> Result<(), String> {
    with_write_state(|state| {
        match state.product.remove(&key) {
            Some(_) => Ok(()), 
            None => Err(String::from(crate::utils::constants::ERROR_PROMO_NOT_FOUND)), 
        }
    })
}
// this is the controller function to create a product.
// pub fn controller_create_product(
//     key: String,
//     product: crate::models::product::Product, // Change args to product
// ) -> Result<(), String> {
//     with_write_state(|state| {
//         if let Some(mut retailer_profile) = state.retailer.get(&ic_cdk::api::caller()) {
//             if state.product.contains_key(&key.to_string()) {
//                 return Err(String::from(
//                     crate::utils::constants::WARNING_PRODUCT_EXISTS,
//                 ));
//             }
//             if let Some(promo_ids) = &mut retailer_profile.product_id {
//                 promo_ids.push(key.clone().to_string());
//             } else {
//                 retailer_profile.product_id = Some(vec![key.to_string()]);
//             }
//             state.product.insert(key.clone(), product);
//             state
//                 .retailer
//                 .insert(ic_cdk::api::caller(), retailer_profile);
//             Ok(())
//         } else {
//             Err(String::from(
//                 crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
//             ))
//         }
//     })
// }

// // Function to update the product.
// pub fn controller_update_product(
//     key: String,
//     args: crate::models::product::Product,
// ) -> Result<(), String> {
//     with_write_state(|state| {
//         if state.retailer.get(&ic_cdk::api::caller()).is_some() {
//             if let Some(existing_product) = state.product.get(&key) {
//                 let updated_product = crate::models::product::Product {
//                     name: if args.name.is_empty() {
//                         existing_product.name.clone()
//                     } else {
//                         args.name.clone()
//                     },
//                     category_id: if args.category_id == 0 {
//                         existing_product.category_id
//                     } else {
//                         args.category_id
//                     },
//                     brand: if args.brand.is_empty() {
//                         existing_product.brand.clone()
//                     } else {
//                         args.brand.clone()
//                     },
//                     umbrella_brand: if args.umbrella_brand.is_empty() {
//                         existing_product.umbrella_brand.clone()
//                     } else {
//                         args.umbrella_brand.clone()
//                     },
//                     regular_price: if args.regular_price == 0.0 {
//                         existing_product.regular_price
//                     } else {
//                         args.regular_price
//                     },
//                     stock_level: if args.stock_level == 0 {
//                         existing_product.stock_level
//                     } else {
//                         args.stock_level
//                     },
//                     supplier: if args.supplier.is_empty() {
//                         existing_product.supplier.clone()
//                     } else {
//                         args.supplier.clone()
//                     },
//                     description: if args.description.is_empty() {
//                         existing_product.description.clone()
//                     } else {
//                         args.description.clone()
//                     },
//                     start_date: if args.start_date == 0 {
//                         existing_product.start_date
//                     } else {
//                         args.start_date
//                     },
//                     end_date: if args.end_date == 0 {
//                         existing_product.end_date
//                     } else {
//                         args.end_date
//                     },
//                     original_price: if args.original_price == 0.0 {
//                         existing_product.original_price
//                     } else {
//                         args.original_price
//                     },
//                     price_after_promotion: if args.price_after_promotion == 0.0 {
//                         existing_product.price_after_promotion
//                     } else {
//                         args.price_after_promotion
//                     },
//                     promotion_description: if args.promotion_description.is_none() {
//                         existing_product.promotion_description.clone()
//                     } else {
//                         args.promotion_description.clone()
//                     },
//                 };

//                 // Update the stored product
//                 state.product.insert(key.clone(), updated_product);
//                 Ok(())
//             } else {
//                 Err(String::from(
//                     crate::utils::constants::ERROR_PRODUCT_NOT_REGISTERED,
//                 ))
//             }
//         } else {
//             Err(String::from(
//                 crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
//             ))
//         }
//     })
// }
