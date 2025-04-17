pub enum UserIndicator {
    SelectProduct,
    ViewSelectedProduct,
    DeleteFromSelectedProduct,
    EditFromSelectedProduct,
    ViewListedProduct,
    GoBack,
}

impl UserIndicator {
    pub fn user_show() {
        println!("==");
        println!("==");
        println!("1. Select Product");
        println!("2. View Selected Product");
        println!("3. Delete Selected Product");
        println!("4. Edit Selected Product");
        println!("5. View Store Product list.");
        println!("6. Return to Previous Menu");
        println!("==");
        println!("==")
    }

    pub fn user_indicator(num: i32) -> Option<Self> {
        match num {
            1 => Some(Self::SelectProduct),
            2 => Some(Self::ViewSelectedProduct),
            3 => Some(Self::DeleteFromSelectedProduct),
            4 => Some(Self::EditFromSelectedProduct),
            5 => Some(Self::ViewListedProduct),
            6 => Some(Self::GoBack),
            _ => None,
        }
    }
}
