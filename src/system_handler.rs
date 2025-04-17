use crate::{
    operation::{inventory::Inventory, system_operation::SystemHandler},
    store::store_model::StoreModel,
    users::buyer_model::BuyerModel,
    utils::get_input,
};

pub fn run_system<T, U, W>(store: &mut U, buyer: &mut W) -> Result<(), String>
where
    T: SystemHandler<U, W>,
    U: Inventory<StoreModel>,
    W: Inventory<BuyerModel>,
{
    loop {
        T::show_meun();

        let input: i32 = get_input("Enter Selection:")?;

        if let Some(option) = T::parse_input(input) {
            let should_exit = T::handle_option(option, store, buyer)?;

            if should_exit {
                return Ok(());
            }
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}
