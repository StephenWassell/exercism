use super::attrs::Attrs;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    from: String,
    to: String,
    attrs: Attrs,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
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
