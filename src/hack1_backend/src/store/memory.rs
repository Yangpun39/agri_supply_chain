use std::cell::RefCell;

use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager}, DefaultMemoryImpl};

const AGRICULTURE_DATA: MemoryId = MemoryId::new(0);
const PRODUCT_DATA: MemoryId = MemoryId::new(1);
const RETAILER_DATA: MemoryId = MemoryId::new(2);
const CUSTOMER_DATA:MemoryId=MemoryId::new(3);
const FARMER_DATA:MemoryId=MemoryId::new(4);
const DISTRIBUTOR_DATA:MemoryId=MemoryId::new(5);
const STORE_DATA:MemoryId=MemoryId::new(6);

pub type StoreMemory = ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;


thread_local! {
  static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
      RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn get_agriculture_data_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(AGRICULTURE_DATA))
}
pub fn get_product_data_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(PRODUCT_DATA))
}

pub fn get_retailer_data_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(RETAILER_DATA))
}
pub fn get_customer_data_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(CUSTOMER_DATA))
}
pub fn get_farmer_data_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(FARMER_DATA))
}
// this is our function for the user.
pub fn get_distributor_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(DISTRIBUTOR_DATA))
}
pub fn get_store_data_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(STORE_DATA))
}

