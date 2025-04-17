use crate::{
    operation::{inventory::Inventory, system_operation::SystemHandler},
    store::{delete_product, store_model::StoreModel, view_products},
};

use super::{
    buy_product, buyer_model::BuyerModel, update_bought_product, user_indicator::UserIndicator,
};

pub struct BuyerSystem;

impl<T, U> SystemHandler<T, U> for BuyerSystem
where
    T: Inventory<StoreModel>,
    U: Inventory<BuyerModel>,
{
    type MenuOption = UserIndicator;

    fn show_meun() {
        UserIndicator::user_show()
    }

    fn parse_input(input: i32) -> Option<Self::MenuOption> {
        UserIndicator::user_indicator(input)
    }

    fn handle_option(
        option: Self::MenuOption,
        store: &mut T,
        buyer: &mut U,
    ) -> Result<bool, String> {
        match option {
            UserIndicator::SelectProduct => buy_product(buyer, store)?,
            UserIndicator::ViewSelectedProduct => view_products(buyer),
            UserIndicator::DeleteFromSelectedProduct => delete_product(buyer)?,
            UserIndicator::EditFromSelectedProduct => update_bought_product(buyer, store)?,
            UserIndicator::ViewListedProduct => view_products(store),
            UserIndicator::GoBack => return Ok(true),
        }

        Ok(false)
    }
}
