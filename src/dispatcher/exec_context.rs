use std::fmt::Debug;
use fnv::FnvHashMap;

#[derive(Debug)]
pub struct ExecContext<C: Debug> {
    arguments: FnvHashMap<String, String>,
    context: C
}

impl<C: Debug> ExecContext<C> {
    pub fn new(context: C) -> Self {
        Self {
            arguments: FnvHashMap::default(),
            context
        }
    }
}
