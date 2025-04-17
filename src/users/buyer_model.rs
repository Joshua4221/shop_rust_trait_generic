use crate::operation::info_model::ProductInfo;

pub struct BuyerModel {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

impl ProductInfo for BuyerModel {
    fn name(&self) -> &String {
        &self.name
    }

    fn price(&self) -> &f64 {
        &self.price
    }

    fn quantity(&self) -> &i32 {
        &self.quantity
    }

    fn mut_price(&mut self) -> &mut f64 {
        &mut self.price
    }

    fn mut_quantity(&mut self) -> &mut i32 {
        &mut self.quantity
    }

    fn from_input(name: String, price: f64, quantity: i32) -> Self {
        Self {
            name,
            price,
            quantity,
        }
    }
}
