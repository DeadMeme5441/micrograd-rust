mod engine;
mod nn;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;

use engine::*;
use nn::*;

fn main() {
    // test_engine();
    // test_nn();
}

fn test_nn() {
    let n = MLP::new(3, &mut [4, 4, 1].to_vec());

    let mut xs = [
        [NNode::new(2.0), NNode::new(3.0), NNode::new(-1.0)].to_vec(),
        [NNode::new(3.0), NNode::new(-1.0), NNode::new(0.5)].to_vec(),
        [NNode::new(0.5), NNode::new(1.0), NNode::new(1.0)].to_vec(),
        [NNode::new(1.0), NNode::new(1.0), NNode::new(-1.0)].to_vec(),
    ]
    .to_vec();

    let mut ys = [
        NNode::new(1.0),
        NNode::new(-1.0),
        NNode::new(-1.0),
        NNode::new(1.0),
    ]
    .to_vec();

    let mut y_pred: Vec<NNode> = Vec::new();
    let mut loss: NNode = NNode::new(0.0);

    for k in 0..20 {
        // Forward Pass
        y_pred = xs
            .iter_mut()
            .map(|x| n.call(&x).into_iter().next().unwrap())
            .collect::<Vec<NNode>>();

        loss = add_vec(
            &zip(&mut ys, &mut y_pred)
                .map(|(mut ygt, yout)| (yout.sub(&mut ygt)).pow(&NNode::new(2.0)))
                .collect::<Vec<NNode>>(),
        );

        // Backward Pass
        for para in n.parameters() {
            para.root.as_ref().borrow_mut().grad = 0.0;
        }
        loss.backwards();

        // Update
        for para in n.parameters() {
            let mut p = para.root.as_ref().borrow_mut();
            p.data += -0.1 * p.grad;
        }

        println!("{:?} {:?}", k, loss.root.as_ref().borrow().data);
    }
    println!("{:?}", y_pred);

    loss.visualise();
    // println!("{:?}", n.parameters());
}
fn test_engine() {
    let x1: NNode = NNode::init(2.0, "x1".to_string());
    let x2: NNode = NNode::init(0.0, "x2".to_string());

    let w1: NNode = NNode::init(-3.0, "w1".to_string());
    let w2: NNode = NNode::init(1.0, "w2".to_string());

    let x1w1 = x1.mul(&w1);
    x1w1.root.as_ref().borrow_mut().label = "x1w1".to_string();
    let x2w2 = x2.mul(&w2);
    x2w2.root.as_ref().borrow_mut().label = "x2w2".to_string();

    let b: NNode = NNode::init(6.8813735870195432, "b".to_string());

    let x1w1x2w2 = x1w1.add(&x2w2);
    x1w1x2w2.root.as_ref().borrow_mut().label = "x1w1+x2w2".to_string();
    // let mut m = add_vec(&[x1w1, x2w2, b].to_vec());

    let m = x1w1x2w2.add(&b);
    m.root.as_ref().borrow_mut().label = "m".to_string();

    let o = m.tanh();
    o.root.as_ref().borrow_mut().label = "o".to_string();
    o.root.as_ref().borrow_mut().grad = 1.0;

    o.backwards();

    o.visualise();
}
