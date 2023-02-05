use core::fmt;
use dyn_clonable::*;
use std::cell::RefCell;
use std::ops::{Add, Mul};
use std::rc::Rc;

#[clonable]
trait MyTrait: Clone {
    fn clone(&self);
}

#[derive(Clone)]
pub struct Value {
    pub data: f64,
    pub prev: Vec<Rc<RefCell<Value>>>,
    pub op: String,
    pub label: String,
    pub grad: f64,
}

impl Value {
    pub fn init(data: f64, label: String) -> Self {
        Self {
            data,
            prev: Vec::new(),
            op: "".to_string(),
            label,
            grad: 0.0,
        }
    }

    pub fn back(self) {
        if self.prev.len() != 0 {
            let mut this = self.prev[0].borrow_mut();
            let mut other = self.prev[1].borrow_mut();
            match self.op.as_str() {
                "+" => {
                    println!("Printing add");
                    println!("{:?}", self.grad);
                    this.grad += 1.0 * self.grad;
                    other.grad += 1.0 * self.grad;
                }
                "*" => {
                    println!("Printing multiply");
                    println!("{:?}", self.grad);
                    this.grad += other.data * self.grad;
                    other.grad += this.data * self.grad;
                }
                "" => {}
                _ => {}
            }
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
        write!(
            f,
            "Value(label={}\n,data={}\n, gradient={}\n, prev={:?})\n\n",
            self.label, self.data, self.grad, self.prev
        )
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let this = Rc::new(RefCell::new(self));
        let other = Rc::new(RefCell::new(other));

        let out: Value = Value {
            data: this.borrow_mut().data + other.borrow_mut().data,
            prev: Vec::from([Rc::clone(&this), Rc::clone(&other)]),
            op: "+".to_string(),
            label: "".to_string(),
            grad: 0.0,
        };

        out
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let this = Rc::new(RefCell::new(self));
        let other = Rc::new(RefCell::new(other));

        let out: Value = Value {
            data: this.borrow_mut().data + other.borrow_mut().data,
            prev: Vec::from([Rc::clone(&this), Rc::clone(&other)]),
            op: "*".to_string(),
            label: "".to_string(),
            grad: 0.0,
        };

        out
    }
}

impl Value {
    fn build_topo(&self, topo: &mut Vec<Value>, visited: &mut Vec<Value>, v: Rc<RefCell<Value>>) {
        let val = v.borrow_mut();
        if !visited.contains(&val) {
            visited.push(val.clone());
            for child in val.prev.iter() {
                self.build_topo(topo, visited, child.clone());
            }
            topo.push(val.clone());
        }
    }

    pub fn backwards(self) {
        let obj = Rc::new(RefCell::new(self.clone()));
        let mut topo: Vec<Value> = Vec::new();
        let mut visited: Vec<Value> = Vec::new();

        self.build_topo(&mut topo, &mut visited, obj);

        topo.reverse();

        for v in topo {
            v.back();
        }

        // println!("{:?}", &topo);
    }
}
