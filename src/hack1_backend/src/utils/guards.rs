use crate::with_read_state;

// pub fn guard_only_agriculture() -> Result<(), String> {
//         match id{
//             Ok(admin_id) if admin_id == caller().to_text() => Ok(()),
//             _ => Err(String::from(crate::utils::constants::WARNING_ANONYMOUS_CALL)),
//         }
// }

pub fn guard_check_admin() -> Result<(), String> {
    with_read_state(|state| {
        match state.user.get(&ic_cdk::api::caller()) {
            Some(role) if role == "admin" => Ok(()),  // Check if the role is "admin"
            _ => Err(String::from(crate::utils::constants::WARNING_ANONYMOUS_CALL)),
        }
    })
}
// pub fn guard_prevent_anonymous_retailer() -> Result<(), String> {
//     with_read_state(|state| { 
//         if state.retailer.contains_key(&ic_cdk::api::caller()) {
//             Ok(()) 
//         } else {
//             Err(String::from(crate::utils::constants::WARNING_ANONYMOUS_CALL)) 
//         }
//     })
// }
pub fn guard_prevent_user_recreation()-> Result<(), String> {
    with_read_state(|state| { 
        if !state.user.contains_key(&ic_cdk::api::caller()) {
              Err(String::from(crate::utils::constants::WARNING_ANONYMOUS_CALL)) 
        } else {
            Ok(())
        }
    })
}