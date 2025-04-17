use crate::{
    company::company_indicator::CompanyIndicator,
    operation::inventory::Inventory,
    store::{admin_system::AdminSystem, store_model::StoreModel},
    system_handler::run_system,
    users::{buyer_model::BuyerModel, buyer_system::BuyerSystem},
    utils::get_input,
};

pub fn authentication<T: Inventory<StoreModel>, U: Inventory<BuyerModel>>() -> Result<(), String> {
    let mut store = T::new();
    let mut shoped_items = U::new();

    loop {
        CompanyIndicator::company_show();

        let auth_input: i32 = get_input("Enter User Type")?;

        match CompanyIndicator::company_indicator(auth_input) {
            Some(CompanyIndicator::Admin) => {
                run_system::<AdminSystem, T, U>(&mut store, &mut shoped_items)?
            }
            Some(CompanyIndicator::User) => {
                run_system::<BuyerSystem, T, U>(&mut store, &mut shoped_items)?
            }
            None => return Ok(()),
        }
    }
}
