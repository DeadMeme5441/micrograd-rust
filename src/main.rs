mod engine;
use std::cell::RefCell;
use std::rc::Rc;

use engine::*;

fn main() {
    let x1: Value = Value::init(2.0, "x1".to_string());
    let x2: Value = Value::init(0.0, "x2".to_string());

    let w1: Value = Value::init(-3.0, "w1".to_string());
    let w2: Value = Value::init(1.0, "w2".to_string());

    let mut x1w1 = x1.mul(&w1);
    x1w1.label = "x1w1".to_string();
    let mut x2w2 = x2.mul(&w2);
    x2w2.label = "x2w2".to_string();

    let b: Value = Value::init(6.8813735870195432, "b".to_string());

    let mut x1w1x2w2 = x1w1.add(&x2w2);
    x1w1x2w2.label = "x1w1+x2w2".to_string();

    let mut m = x1w1x2w2.add(&b);
    m.label = "m".to_string();

    let mut o = m.tanh();
    o.label = "o".to_string();
    o.grad = 1.0;

    let p = NNode {
        root: Rc::new(RefCell::new(o)),
    };

    p.backwards();

    p.visualise();
}
