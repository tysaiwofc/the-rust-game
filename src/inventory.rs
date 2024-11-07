pub struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item.clone());
        println!("Você pegou um item: {}", item);
    }
}
