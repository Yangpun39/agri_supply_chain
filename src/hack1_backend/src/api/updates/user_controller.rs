use crate::with_write_state;



pub fn controller_create_farmer(
    args: String,signature:String
) -> Result<(), String> {
    with_write_state(|state| {
        if state.farmer.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                crate::utils::constants::WARNING_ACCOUNT_EXISTS,
            ));
        } else {
        }
        // let keypair=super::digital_verify::load_keypair_base64(&signing_key);
        let hash_data=crate::api::queries::verify::hash_string_sha256(&args);
        // let sig=super::digital_verify::sign_message(&keypair, hash_data.as_bytes());
        let account=crate::models::farmer::Farmer{
            data:hash_data,
            signature:signature,
            permission:None,
            product_id:None,
        };
        state.farmer.insert(ic_cdk::api::caller(),account);
        state
            .user
            .insert(ic_cdk::api::caller(), "farmer".to_string());

        Ok(())
    })
}

pub fn controller_update_farmer(
    args: crate::models::farmer::Farmer,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(existing_profile) = state.farmer.get(&ic_cdk::api::caller()) {
            // Create an updated profile by keeping existing values for empty or `None` fields
            let updated_profile = crate::models::farmer::Farmer{
                data: if args.data.is_empty() {
                    existing_profile.data.clone()
                } else {
                    args.data.clone()
                },
                signature: if args.signature.is_empty() {
                    existing_profile.signature.clone()
                } else {
                    args.signature.clone()
                },
                product_id: if args.product_id.is_none() {
                    existing_profile.product_id.clone()
                } else {
                    args.product_id.clone()
                },
                permission: if args.permission.is_none() {
                    existing_profile.permission.clone()
                } else {
                    args.permission.clone()
                },
            };

            // Update the stored retailer profile
            state
                .farmer
                .insert(ic_cdk::api::caller(), updated_profile);
            Ok(())
        } else {
            Err(String::from(
                crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}

pub fn controller_create_distributor(
    args: String,signature:String
) -> Result<(), String> {
    with_write_state(|state| {
        if state.distributor.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                crate::utils::constants::WARNING_ACCOUNT_EXISTS,
            ));
        } else {
        }
        //let keypair=super::digital_verify::load_keypair_base64(&signing_key);
        let hash_data=crate::api::queries::verify::hash_string_sha256(&args);
       // let sig=super::digital_verify::sign_message(&keypair, hash_data.as_bytes());
        let account=crate::models::distributor::Distributor{
            data:hash_data,
            signature:signature,
            permission:None,
            product_id:None,
        };
        state.distributor.insert(ic_cdk::api::caller(), account);
        state
            .user
            .insert(ic_cdk::api::caller(), "distributor".to_string());

        Ok(())
    })
}

pub fn controller_update_distributor(
    args: crate::models::distributor::Distributor,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(existing_profile) = state.distributor.get(&ic_cdk::api::caller()) {
            // Create an updated profile by keeping existing values for empty or `None` fields
            let updated_profile = crate::models::distributor::Distributor {
                data: if args.data.is_empty() {
                    existing_profile.data.clone()
                } else {
                    args.data.clone()
                },
                signature: if args.signature.is_empty() {
                    existing_profile.signature.clone()
                } else {
                    args.signature.clone()
                },
                product_id: if args.product_id.is_none() {
                    existing_profile.product_id.clone()
                } else {
                    args.product_id.clone()
                },
                permission: if args.permission.is_none() {
                    existing_profile.permission.clone()
                } else {
                    args.permission.clone()
                },
            };

            // Update the stored retailer profile
            state
                .distributor
                .insert(ic_cdk::api::caller(), updated_profile);
            Ok(())
        } else {
            Err(String::from(
                crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}


pub fn controller_create_retailer(
    args: String,signature:String
) -> Result<(), String> {
    with_write_state(|state| {
        if state.retailer.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                crate::utils::constants::WARNING_ACCOUNT_EXISTS,
            ));
        } else {
        }
     //   let keypair=super::digital_verify::load_keypair_base64(&signing_key);
        let hash_data=crate::api::queries::verify::hash_string_sha256(&args);
       // let sig=super::digital_verify::sign_message(&keypair, hash_data.as_bytes());
        let account=crate::models::retailer_types::RetailerProfile{
            data:hash_data,
            signature:signature,
            permission:None,
            product_id:None,
        };
        state.retailer.insert(ic_cdk::api::caller(), account);
        state
            .user
            .insert(ic_cdk::api::caller(), "retailer".to_string());

        Ok(())
    })
}

pub fn controller_update_retailer(
    args: crate::models::retailer_types::RetailerProfile,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(existing_profile) = state.retailer.get(&ic_cdk::api::caller()) {
            // Create an updated profile by keeping existing values for empty or `None` fields
            let updated_profile = crate::models::retailer_types::RetailerProfile {
                data: if args.data.is_empty() {
                    existing_profile.data.clone()
                } else {
                    args.data.clone()
                },
                signature: if args.signature.is_empty() {
                    existing_profile.signature.clone()
                } else {
                    args.signature.clone()
                },
                product_id: if args.product_id.is_none() {
                    existing_profile.product_id.clone()
                } else {
                    args.product_id.clone()
                },
                permission: if args.permission.is_none() {
                    existing_profile.permission.clone()
                } else {
                    args.permission.clone()
                },
            };

            // Update the stored retailer profile
            state
                .retailer
                .insert(ic_cdk::api::caller(), updated_profile);
            Ok(())
        } else {
            Err(String::from(
                crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}

// //store creation
// // pub fn controller_store_retailer(
// //     key: &str,
// //     args: crate::models::store_detail::StoreDetail,
// // ) -> Result<(), String> {
// //     with_write_state(|state| {
// //         if let Some(mut retailer_profile) = state.retailer.get(&ic_cdk::api::caller()) {
// //             if state.store.contains_key(&key.to_string()) {
// //                 return Err(String::from(crate::utils::constants::WARNING_STORE_EXISTS));
// //             }
// //             state.store.insert(key.to_string(), args);
// //             if let Some(store_ids) = &mut retailer_profile.store_id {
// //                 store_ids.push(key.to_string());
// //             } else {
// //                 retailer_profile.store_id = Some(vec![key.to_string()]);
// //             }
// //             state
// //                 .retailer
// //                 .insert(ic_cdk::api::caller(), retailer_profile);
// //             Ok(())
// //         } else {
// //             Err(String::from(
// //                 crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
// //             ))
// //         }
// //     })
// // }

// //store upgradation
// // pub fn controller_store_retailer_update(
// //     key: String,
// //     args: crate::models::store_detail::StoreDetail,
// // ) -> Result<(), String> {
// //     with_write_state(|state| {
// //         if state.retailer.get(&ic_cdk::api::caller()).is_some() {
// //             if let Some(existing_store) = state.store.get(&key.to_string()) {
// //                 let updated_store = crate::models::store_detail::StoreDetail {
// //                     store_type: if args.store_type.is_empty() {
// //                         existing_store.store_type.clone()
// //                     } else {
// //                         args.store_type.clone()
// //                     },
// //                     country: if args.country.is_empty() {
// //                         existing_store.country.clone()
// //                     } else {
// //                         args.country.clone()
// //                     },
// //                     state: if args.state.is_empty() {
// //                         existing_store.state.clone()
// //                     } else {
// //                         args.state.clone()
// //                     },
// //                     postal_code: if args.postal_code.is_empty() {
// //                         existing_store.postal_code.clone()
// //                     } else {
// //                         args.postal_code.clone()
// //                     },
// //                     city: if args.city.is_empty() {
// //                         existing_store.city.clone()
// //                     } else {
// //                         args.city.clone()
// //                     },
// //                     phone_number: if args.phone_number.is_empty() {
// //                         existing_store.phone_number.clone()
// //                     } else {
// //                         args.phone_number.clone()
// //                     },
// //                     email: if args.email.is_empty() {
// //                         existing_store.email.clone()
// //                     } else {
// //                         args.email.clone()
// //                     },
// //                     total_product: if args.total_product == 0 {
// //                         existing_store.total_product
// //                     } else {
// //                         args.total_product
// //                     },
// //                     total_supplier: if args.total_supplier == 0 {
// //                         existing_store.total_supplier
// //                     } else {
// //                         args.total_supplier
// //                     },
// //                     total_promotion: if args.total_promotion == 0 {
// //                         existing_store.total_promotion
// //                     } else {
// //                         args.total_promotion
// //                     },
// //                     total_sales: if args.total_sales == 0 {
// //                         existing_store.total_sales
// //                     } else {
// //                         args.total_sales
// //                     },
// //                     total_units_sold: if args.total_units_sold == 0 {
// //                         existing_store.total_units_sold
// //                     } else {
// //                         args.total_units_sold
// //                     },
// //                     top_selling_product: if args.top_selling_product.is_empty() {
// //                         existing_store.top_selling_product.clone()
// //                     } else {
// //                         args.top_selling_product.clone()
// //                     },
// //                 };

// //                 // Update the stored store profile
// //                 state.store.insert(key.clone(), updated_store);
// //                 Ok(())
// //             } else {
// //                 Err(String::from(crate::utils::constants::ERROR_STORE_NOT_REGISTERED))
// //             }
// //         } else {
// //             Err(String::from(
// //                 crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
// //             ))
// //         }
// //     })
// // }
