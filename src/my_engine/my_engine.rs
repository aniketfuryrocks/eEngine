use crate::my_engine::{physics::* ,app::*};

use std::collections::*;

pub struct MyEngine{
    pub app :App,
    pub objects:HashMap<String,Box<dyn Object>>
}

impl MyEngine {
    pub fn new(app:App)->MyEngine{
        MyEngine {
            app : app,
            objects : HashMap::new()
        }
    }
}