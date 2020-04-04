use super::attrs::Attrs;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    attrs: Attrs,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: Attrs::new(),
        }
    }

    pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        let mut ret = self.clone();
        ret.attrs.insert(attrs);
        ret
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name)
    }
}
