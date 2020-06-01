use std::collections::HashMap;

struct DB {
    dao: HashMap<u16, String>
    instance_existed : bool
}

impl DB {
    fn get_instance() -> Self {
        Self {
            dao: HashMap::new(),
            instance_existed :: true
        }
    }
}

fn main() {
    DB::get_instance()
}