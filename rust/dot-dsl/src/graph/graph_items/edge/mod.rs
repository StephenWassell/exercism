use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    from: String,
    to: String,
    attrs: super::super::Attrs,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
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
