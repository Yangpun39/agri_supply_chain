use ic_stable_structures::StableBTreeMap;
use std::string::String;

use crate::models::distributor::Distributor;
use crate::models::farmer::Farmer;
use crate::models::product::Product;
use crate::models::customer::Customer;
use crate::models::retailer_types::RetailerProfile;
use crate::models::agriculture::Agriculture;


use super::memory::StoreMemory;

pub(crate) struct ApplicationState {
    pub agriculture_department: StableBTreeMap<candid::Principal, Agriculture, StoreMemory>,
    pub retailer: StableBTreeMap<candid::Principal, RetailerProfile, StoreMemory>,
    pub customer: StableBTreeMap<candid::Principal, Customer, StoreMemory>,
    pub product: StableBTreeMap<String, Product, StoreMemory>,
    pub farmer: StableBTreeMap<candid::Principal, Farmer, StoreMemory>,
    pub distributor: StableBTreeMap<candid::Principal, Distributor, StoreMemory>,
    pub user:StableBTreeMap<candid::Principal, String, StoreMemory>,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            agriculture_department: init_agriculture_state(),
            product: product_state(),
            retailer: init_retailer_state(),
            customer: init_customer_state(),
            farmer: farmer_state(),
            distributor: distributor_state(),
            user:init_user_state(),
        }
    }
}

fn init_agriculture_state() -> StableBTreeMap<candid::Principal,  Agriculture, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_agriculture_data_memory())
}
fn product_state() -> StableBTreeMap<String, Product, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_product_data_memory())
}
fn init_retailer_state() -> StableBTreeMap<candid::Principal, RetailerProfile, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_retailer_data_memory())
}
fn init_customer_state() -> StableBTreeMap<candid::Principal, Customer, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_customer_data_memory())
}
fn farmer_state() -> StableBTreeMap<candid::Principal, Farmer, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_farmer_data_memory())
}
// user function ---> to use.
fn distributor_state() -> StableBTreeMap<candid::Principal, Distributor, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_distributor_memory())
}
//for store detail in retailer
fn init_user_state() -> StableBTreeMap<candid::Principal, String, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_store_data_memory())
}
