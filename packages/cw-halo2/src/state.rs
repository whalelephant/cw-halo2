use cw_storage_plus::{Item, Map};

pub const STATE: Item<bool> = Item::new("some_state");
