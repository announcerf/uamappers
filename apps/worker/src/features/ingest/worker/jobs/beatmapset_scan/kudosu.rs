use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct KudosuCache {
    values: HashMap<u32, (i32, i32)>,
}

impl KudosuCache {
    pub fn get(&self, user_id: u32) -> Option<(i32, i32)> {
        self.values.get(&user_id).copied()
    }

    pub fn insert(&mut self, user_id: u32, available: i32, total: i32) {
        self.values.insert(user_id, (available, total));
    }
}
