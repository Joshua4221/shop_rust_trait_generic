use crate::{
    operation::{inventory::Inventory, system_operation::SystemHandler},
    users::buyer_model::BuyerModel,
};

use super::{
    create_product, delete_product, store_indicator::StoreIndicator, store_model::StoreModel,
    update_product, view_products,
};

pub struct AdminSystem;

impl<T, U> SystemHandler<T, U> for AdminSystem
where
    T: Inventory<StoreModel>,
    U: Inventory<BuyerModel>,
{
    type MenuOption = StoreIndicator;

    fn show_meun() {
        StoreIndicator::admin_show();
    }

    fn parse_input(input: i32) -> Option<Self::MenuOption> {
        StoreIndicator::indicator(input)
    }

    fn handle_option(
        option: Self::MenuOption,
        store: &mut T,
        buyer: &mut U,
    ) -> Result<bool, String> {
        match option {
            StoreIndicator::CreateProduct => create_product(store)?,
            StoreIndicator::ViewProduct => view_products(store),
            StoreIndicator::DeleteProduct => delete_product(store)?,
            StoreIndicator::UpdateProduct => update_product(store)?,
            StoreIndicator::ViewBuyerOrder => view_products(buyer),
            StoreIndicator::GoBack => return Ok(true),
        }

        Ok(false)
    }
}
