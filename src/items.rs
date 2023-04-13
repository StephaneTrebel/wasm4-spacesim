#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Item {
    IronIngot,
    FoodCrate,
}

impl Item {
    pub fn get_name(&self) -> &str {
        match self {
            Item::IronIngot => "Iron ingot",
            Item::FoodCrate => "Food crate",
        }
    }
}
