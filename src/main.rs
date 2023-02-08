mod engine;
use std::cell::RefCell;
use std::rc::Rc;

use engine::*;

fn main() {
    let a: Value = Value::init(2.0, "a".to_string());
    let b: Value = Value::init(-3.0, "b".to_string());
    let c: Value = Value::init(10.0, "c".to_string());

    let mut e = a.mul(&b);
    e.label = "e".to_string();

    let mut d = e.add(&c);
    d.label = "d".to_string();

    let f = Value::init(-2.0, "f".to_string());

    let mut l = d.mul(&f);

    l.label = "L".to_string();
    l.grad = 1.0;

    let n = NNode {
        root: Rc::new(RefCell::new(l)),
    };

    n.backwards();

    n.visualise();
}
