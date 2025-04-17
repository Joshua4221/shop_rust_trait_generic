pub enum CompanyIndicator {
    Admin,
    User,
}

impl CompanyIndicator {
    pub fn company_show() {
        println!("==");
        println!("==");
        println!("select user type");
        println!("1. Shop Admin.");
        println!("2. User.");
        println!("==");
        println!("==");
    }

    pub fn company_indicator(num: i32) -> Option<Self> {
        match num {
            1 => Some(Self::Admin),
            2 => Some(Self::User),
            _ => None,
        }
    }
}
