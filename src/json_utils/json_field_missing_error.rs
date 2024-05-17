use std::fmt::{Debug, Formatter};

pub struct JSONFieldMissingError {
    pub field_name: String
}

impl Debug for JSONFieldMissingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field '{}' is missing!", self.field_name)
    }
}
