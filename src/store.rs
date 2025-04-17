pub mod admin_system;
pub mod store_indicator;
pub mod store_model;
pub mod store_schema;

use crate::{
    operation::{info_model::ProductInfo, inventory::Inventory},
    utils::get_input,
};

pub fn create_product<T, U>(store: &mut T) -> Result<(), String>
where
    T: Inventory<U>,
    U: ProductInfo,
{
    let name: String = get_input("Enter Name")?;

    let price: f64 = get_input("Enter Price")?;

    let quantity: i32 = get_input("Enter Quantity")?;

    let product = U::from_input(name, price, quantity);

    store.add(product)?;

    Ok(())
}

pub fn view_products<T, U>(store: &mut T)
where
    T: Inventory<U>,
    U: ProductInfo,
{
    if store.get().is_empty() {
        println!("store is empty");
    }

    let store_items: Vec<&U> = store.get();

    for item in store_items {
        println!(
            "{} of {} is of {} quantity",
            item.name(),
            item.price(),
            item.quantity()
        );
    }
}

pub fn delete_product<T, U>(store: &mut T) -> Result<(), String>
where
    T: Inventory<U>,
    U: ProductInfo,
{
    if store.get().is_empty() {
        println!("store is empty");
    }

    let store_items = store.get();

    for item in store_items {
        println!(
            "{} of {} is of {} quantity",
            item.name(),
            item.price(),
            item.quantity()
        );
    }

    let name: String = get_input("Enter Product Name:")?;

    store.delete(&name)?;

    Ok(())
}

pub fn update_product<T, U>(store: &mut T) -> Result<(), String>
where
    T: Inventory<U>,
    U: ProductInfo,
{
    if store.get().is_empty() {
        println!("store is empty");
    }

    let store_items = store.get();

    for item in store_items {
        println!(
            "{} of {} is of {} quantity",
            item.name(),
            item.price(),
            item.quantity()
        );
    }

    let name: String = get_input("Enter Product Name:")?;

    let price: f64 = get_input("Enter Product Price:")?;

    let quantity: i32 = get_input("Enter Product Quantity:")?;

    store.update(&name, price, quantity)?;

    Ok(())
}
