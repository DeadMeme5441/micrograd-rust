use core::fmt;
use std::cell::RefCell;
use std::f64::consts::E;
use std::rc::Rc;

use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::exec;
use graphviz_rust::printer::PrinterContext;

#[derive(Clone)]
pub struct Value {
    pub data: f64,
    pub prev: Vec<Rc<RefCell<Value>>>,
    pub op: String,
    pub label: String,
    pub grad: f64,
}

#[derive(Debug)]
pub struct NNode {
    pub root: Rc<RefCell<Value>>,
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

    pub fn add(&self, other: &Self) -> Self {
        let out: Value = Value {
            data: self.data + other.data,
            prev: Vec::from([
                Rc::new(RefCell::new(self.clone())),
                Rc::new(RefCell::new(other.clone())),
            ]),
            op: "+".to_string(),
            label: "".to_string(),
            grad: 0.0,
        };

        out
    }

    pub fn mul(&self, other: &Self) -> Self {
        let out: Value = Value {
            data: self.data * other.data,
            prev: Vec::from([
                Rc::new(RefCell::new(self.clone())),
                Rc::new(RefCell::new(other.clone())),
            ]),
            op: "*".to_string(),
            label: "".to_string(),
            grad: 0.0,
        };

        out
    }

    pub fn tanh(&self) -> Self {
        let out: Value = Value {
            data: ((E.powf(2.0 * self.data) - 1.0) / (E.powf(2.0 * self.data) + 1.0)),
            prev: Vec::from([Rc::new(RefCell::new(self.clone()))]),
            op: "tanh".to_string(),
            label: "".to_string(),
            grad: 0.0,
        };

        out
    }
}

pub fn back(root: Rc<RefCell<Value>>) {
    let bind = root.borrow_mut();
    let l = bind.prev.len();
    let op = &bind.op;
    let grad = bind.grad;
    if l != 0 {
        match op.as_str() {
            "+" => {
                let item1 = &mut bind.prev[0].borrow_mut();
                let item2 = &mut bind.prev[1].borrow_mut();

                item1.grad += 1.0 * grad;
                item2.grad += 1.0 * grad;
            }
            "*" => {
                let item1 = &mut bind.prev[0].borrow_mut();
                let item2 = &mut bind.prev[1].borrow_mut();

                item1.grad += item2.data * grad;
                item2.grad += item1.data * grad;
            }
            "tanh" => {
                let item1 = &mut bind.prev[0].borrow_mut();
                item1.grad += (1.0 - bind.data.powf(2.0)) * grad;
            }
            "" => {}
            _ => {}
        }
    }
}

fn trace(
    root: Rc<RefCell<Value>>,
) -> (
    Vec<Rc<RefCell<Value>>>,
    Vec<(Rc<RefCell<Value>>, Rc<RefCell<Value>>)>,
) {
    let mut nodes: Vec<Rc<RefCell<Value>>> = Vec::new();
    let mut edges: Vec<(Rc<RefCell<Value>>, Rc<RefCell<Value>>)> = Vec::new();

    build(root, &mut nodes, &mut edges);

    (nodes, edges)
}

fn build(
    v: Rc<RefCell<Value>>,
    nodes: &mut Vec<Rc<RefCell<Value>>>,
    edges: &mut Vec<(Rc<RefCell<Value>>, Rc<RefCell<Value>>)>,
) {
    let val = Rc::clone(&v);
    if !nodes.contains(&v) {
        nodes.push(Rc::clone(&v));
    }
    for child in v.as_ref().borrow().prev.iter() {
        edges.push((Rc::clone(&child), Rc::clone(&v)));
        build(Rc::clone(&child), nodes, edges);
    }
}

impl NNode {
    pub fn visualise(&self) {
        let root = &self.clone().root;
        let (nodes, edges) = (trace(root.clone()).0, trace(root.clone()).1);

        let mut dot: Graph =
            graph!(strict di id!("micro"); attr!("rankdir", "LR"), attr!("fontcolor", "red"));

        for item in nodes.iter() {
            let n = item.as_ref().borrow();
            let uid: Id = id!(n.label);
            let text: String = String::from(
                n.label.to_string() + "|" + &n.data.to_string() + "|" + &n.grad.to_string(),
            );
            dot.add_stmt(stmt!(
                node!(esc uid;  attr!(esc "label",esc &text), attr!("shape", "record"))
            ));

            if n.op != "".to_string() {
                let temp_uid: String = n.label.to_string() + &n.op.to_string();
                dot.add_stmt(stmt!(node!(esc temp_uid;attr!("label",esc n.op))));
                dot.add_stmt(stmt!(edge!(node_id!(esc temp_uid) => node_id!(esc uid))))
            }

            for (val1, val2) in edges.iter() {
                let n1 = val1.as_ref().borrow();
                let n2 = val2.as_ref().borrow();
                dot.add_stmt(stmt!(edge!(node_id!(esc r#n1.label.to_string()) => node_id!(esc r#n2.label.to_string() + &n2.op.to_string()))));
            }
        }

        // println!("{:#?}", dot);

        let mut ctx = PrinterContext::default();
        let empty = exec(
            dot,
            &mut ctx,
            vec![
                CommandArg::Format(Format::Svg),
                CommandArg::Output("output.svg".to_string()),
            ],
        );
        println!("{:?}", empty);
    }

    fn build_topo(
        &self,
        topo: &mut Vec<Rc<RefCell<Value>>>,
        visited: &mut Vec<Rc<RefCell<Value>>>,
        v: Rc<RefCell<Value>>,
    ) {
        let val = Rc::clone(&v);
        if !visited.contains(&val) {
            visited.push(Rc::clone(&v));
            for child in val.as_ref().borrow().prev.iter() {
                self.build_topo(topo, visited, Rc::clone(&child));
            }
            topo.push(val);
        }
    }

    pub fn backwards(&self) {
        let obj = Rc::clone(&self.root);
        let mut topo: Vec<Rc<RefCell<Value>>> = Vec::new();
        let mut visited: Vec<Rc<RefCell<Value>>> = Vec::new();

        self.build_topo(&mut topo, &mut visited, obj);

        topo.reverse();

        for v in topo {
            back(v);
        }

        // println!("{:?}", &topo);
    }
}
