pub trait ProductInfo {
    fn name(&self) -> &String;
    fn price(&self) -> &f64;
    fn quantity(&self) -> &i32;

    fn mut_price(&mut self) -> &mut f64;
    fn mut_quantity(&mut self) -> &mut i32;

    fn from_input(name: String, price: f64, quantity: i32) -> Self
    where
        Self: Sized;
}
