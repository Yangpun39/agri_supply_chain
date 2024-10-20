use crate::with_write_state;

pub fn controller_create_account(
    args: crate::models::agriculture::Agriculture
) -> Result<(), String> {
    with_write_state(|state| {
        if state.agriculture_department.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                crate::utils::constants::WARNING_ACCOUNT_EXISTS,
            ));
        } 
        // let (private,public)=super::digital_verify::generate_keypair();
        // let hash_data=crate::api::queries::verify::hash_string_sha256(&args);
        // let account=crate::models::agriculture::Agriculture{
        //     data:hash_data,
        //     verifying_key:public,
        //     signing_key:private
        // };

        state.agriculture_department.insert(ic_cdk::api::caller(), args);
        state
        .user
        .insert(ic_cdk::api::caller(), "admin".to_string());

        Ok(())
    })
}
pub fn controller_update_account(
    args: crate::models::agriculture::Agriculture,
) -> Result<(), String> {
    with_write_state(|state| {
        if let Some(existing_profile) = state.agriculture_department.get(&ic_cdk::api::caller()) {
            let updated_profile = crate::models::agriculture::Agriculture {
                data: if args.data.is_empty() {
                    existing_profile.data.clone()
                } else {
                    args.data.clone()
                },
                signing_key: if args.signing_key.is_empty() {
                    existing_profile.signing_key.clone()
                } else {
                    args.signing_key.clone()
                },
                verifying_key: if args.verifying_key.is_empty() {
                    existing_profile.verifying_key.clone()
                } else {
                    args.verifying_key.clone()
                },
            };

            // Update the stored profile
            state.agriculture_department.insert(ic_cdk::api::caller(), updated_profile);
            Ok(())
        } else {
            Err(String::from(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED))
        }
    })
}



pub fn controller_delete_account(args: candid::Principal) -> Result<(), String> {
    with_write_state(|state| {
        match state.user.get(&args) {
            Some(account_type) => {
                match account_type.as_str() {
                    "distributor" => {
                        state.distributor.remove(&args);
                        state.user.remove(&args);
                        Ok(())
                    }
                    "retailer" => {
                        state.retailer.remove(&args);
                         state.user.remove(&args);
                        Ok(())
                    }
                    "farmer" => {
                        state.farmer.remove(&args);
                         state.user.remove(&args);
                        Ok(())
                    }
                    "admin" => Err(String::from("Admin accounts cannot be deleted")),
                    _ => Err(String::from("Unknown account type")),
                }
            }
            None => Err(String::from(crate::utils::constants:: ERROR_ACCOUNT_NOT_REGISTERED)),
        }
    })
}
// // pub fn controller_subscribe_sku(
// //     key: String, value: String
// // ) -> Result<(), String> {
// //     with_write_state(|state| {
// //         if let Some(mut existing_profile) = state.account.get(&ic_cdk::api::caller()) {
// //             match &mut existing_profile.sku_list {
// //                 Some(sku_list) => {
// //                     sku_list.insert(key, value); 
// //                 }
// //                 None => {
// //                     let mut new_sku_list = HashMap::new();
// //                     new_sku_list.insert(key, value);
// //                     existing_profile.sku_list = Some(new_sku_list); 
// //                 }
// //             }
// //             state.account.insert(ic_cdk::api::caller(), existing_profile);
// //             Ok(())
// //         } else {
// //             Err(String::from(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED))
// //         }
// //     })
// // }