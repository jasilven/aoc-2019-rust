use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Debug)]
pub struct F64(f64);

impl F64 {
    pub fn new(f: f64) -> Self {
        F64(f)
    }
    pub fn get(&self) -> f64 {
        self.0
    }
}

impl Deref for F64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for F64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state);
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &F64) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for F64 {}
