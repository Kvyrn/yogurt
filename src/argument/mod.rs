pub mod parser;

pub struct Argument {
    validator: fn(&str) -> bool,
    pub name: String,
    required: bool,
}

impl Argument {
    pub fn new(validator: fn(&str) -> bool, name: String, required: bool) -> Self {
        Self {
            validator,
            name,
            required,
        }
    }

    pub fn matches(&self, sample: &str) -> bool {
        (self.validator)(sample)
    }

    pub fn is_required(&self) -> bool {
        self.required
    }
}
