use std::collections::HashMap;

use crate::operation::{info_model::ProductInfo, inventory::Inventory};

pub struct StoreSchema<T: ProductInfo> {
    store: HashMap<String, T>,
}

impl<T: ProductInfo> Inventory<T> for StoreSchema<T> {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn add(&mut self, product: T) -> Result<(), String> {
        if self.store.contains_key(product.name()) {
            return Err("Product already exists".into());
        }

        self.store.insert(product.name().clone(), product);

        Ok(())
    }

    fn get(&self) -> Vec<&T> {
        self.store.values().collect()
    }

    fn get_single(&mut self, name: &String) -> Option<&T> {
        self.store.get(name)
    }

    fn delete(&mut self, name: &String) -> Result<(), String> {
        self.store
            .remove(name)
            .map(|_| ())
            .ok_or("Product not found".into())
    }

    fn update(&mut self, name: &String, price: f64, quantity: i32) -> Result<(), String> {
        match self.store.get_mut(name) {
            Some(item) => {
                *item.mut_price() = price;
                *item.mut_quantity() = quantity;

                Ok(())
            }
            None => Err("Product not found".into()),
        }
    }
}
