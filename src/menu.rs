pub struct Menu {
    // items: Vec<String>,
    pub options: String,
}
impl Menu {
    pub fn new() -> Menu {
        Menu {
            options: String::from("1. Feed\n2. Play\n3. Sleep\n4. Wash\n5. Exit"),
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Menu::new()
    }
}
