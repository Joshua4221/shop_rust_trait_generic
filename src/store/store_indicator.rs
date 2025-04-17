pub enum StoreIndicator {
    CreateProduct,
    ViewProduct,
    DeleteProduct,
    UpdateProduct,
    ViewBuyerOrder,
    GoBack,
}

impl StoreIndicator {
    pub fn admin_show() {
        println!("==");
        println!("==");
        println!("1. Create Product");
        println!("2. View Product");
        println!("3. Delete Product");
        println!("4. Edit Product");
        println!("5. Buyer Order");
        println!("6. Return to Previous Menu");
        println!("==");
        println!("==")
    }

    pub fn indicator(num: i32) -> Option<Self> {
        match num {
            1 => Some(Self::CreateProduct),
            2 => Some(Self::ViewProduct),
            3 => Some(Self::DeleteProduct),
            4 => Some(Self::UpdateProduct),
            5 => Some(Self::ViewBuyerOrder),
            6 => Some(Self::GoBack),
            _ => None,
        }
    }
}
