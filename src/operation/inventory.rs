pub trait Inventory<T> {
    fn new() -> Self;
    fn add(&mut self, product: T) -> Result<(), String>;
    fn get(&self) -> Vec<&T>;
    fn get_single(&mut self, name: &String) -> Option<&T>;
    fn delete(&mut self, name: &String) -> Result<(), String>;
    fn update(&mut self, name: &String, price: f64, quantity: i32) -> Result<(), String>;
}
