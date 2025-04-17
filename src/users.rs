pub mod buyer_model;
pub mod buyer_system;
pub mod user_indicator;

use buyer_model::BuyerModel;

use crate::{operation::inventory::Inventory, store::store_model::StoreModel, utils::get_input};

pub fn buy_product<T: Inventory<BuyerModel>, U: Inventory<StoreModel>>(
    buyer: &mut T,
    store: &mut U,
) -> Result<(), String> {
    if store.get().is_empty() {
        println!("store is empty");
    }

    let store_items = store.get();

    for item in store_items {
        println!(
            "{} of {} is of {} quantity",
            item.name, item.price, item.quantity
        );
    }

    let name: String = get_input("Enter Product Name")?;

    let product = store
        .get_single(&name)
        .ok_or_else(|| "Product not found.".to_string())?;

    let quantity: i32 = get_input("Enter Quantity")?;

    let main_price = product.price;
    let store_quantity = product.quantity;

    let price = &main_price * quantity as f64;

    if quantity > product.quantity {
        return Err("Quantity selected is more than what's available in the store.".to_string());
    }

    let product = BuyerModel {
        name: name.clone(),
        price,
        quantity,
    };

    buyer.add(product)?;

    store.update(&name, main_price, store_quantity - quantity)?;

    println!("Product bought successfully.");

    Ok(())
}

pub fn update_bought_product<T: Inventory<BuyerModel>, U: Inventory<StoreModel>>(
    buyer: &mut T,
    store: &mut U,
) -> Result<(), String> {
    if buyer.get().is_empty() {
        println!("no product bought currently")
    }

    let buyer_items = buyer.get();

    for item in buyer_items {
        println!(
            "{} of {} is of {} quantity",
            item.name, item.price, item.quantity
        );
    }

    let name: String = get_input("Enter Product Name")?;

    let product = store
        .get_single(&name)
        .ok_or_else(|| "Product not found.".to_string())?;

    let quantity: i32 = get_input("Enter Quantity")?;

    let price = product.price * quantity as f64;

    buyer.update(&name, price, quantity)?;

    Ok(())
}
