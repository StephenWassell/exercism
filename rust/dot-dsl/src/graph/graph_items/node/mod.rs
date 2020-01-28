use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    attrs: super::super::Attrs,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        let mut ret = self.clone();
        for (name, value) in attrs {
            ret.attrs.insert(name.to_string(), value.to_string());
        }
        ret
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        Some(&self.attrs.get(name)?)
    }
}
