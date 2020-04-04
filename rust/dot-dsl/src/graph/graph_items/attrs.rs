use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attrs {
	data: HashMap<String, String>
}

impl Deref for Attrs {
    type Target = HashMap<String, String>;

    fn deref<'a>(&'a self) -> &'a HashMap<String, String> {
        &self.data
    }
}

impl Attrs {
    pub fn new() -> Self {
        Attrs {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, attrs: &[(&str, &str)]) {
        for (name, value) in attrs {
            self.data.insert(name.to_string(), value.to_string());
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        Some(&self.data.get(name)?)
    }
}
