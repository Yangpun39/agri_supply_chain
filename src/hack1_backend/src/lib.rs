use store::storage_state::ApplicationState;
mod api;
mod models;
mod store;
mod utils;


thread_local! {
    static STATE: std::cell::RefCell<ApplicationState> = std::cell::RefCell::new(ApplicationState::new());
}

// to get mutable reference
pub(crate) fn with_write_state<R>(f: impl FnOnce(&mut ApplicationState) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

// to get inmutable reference
pub(crate) fn with_read_state<R>(f: impl FnOnce(&ApplicationState) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

ic_cdk::export_candid!();
