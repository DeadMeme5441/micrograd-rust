use core::fmt;
use std::collections::HashSet;
use std::ops::{Add, Mul, Sub};

#[derive(Clone)]
pub struct Value {
    pub data: f64,
    pub prev: Vec<Value>,
    pub op: String,
    pub label: String,
}

impl Value {
    pub fn init(data: f64, label: String) -> Self {
        Self {
            data,
            prev: Vec::new(),
            op: "".to_string(),
            label,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.label == other.label
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value(data={})", self.data)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: self.data + other.data,
            prev: Vec::from([self, other]),
            op: "+".to_string(),
            label: "".to_string(),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: self.data * other.data,
            prev: Vec::from([self, other]),
            op: "*".to_string(),
            label: "".to_string(),
        }
    }
}
