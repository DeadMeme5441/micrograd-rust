use core::fmt;
use std::cell::{RefCell, RefMut};
use std::f64::consts::E;
use std::rc::Rc;

use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::exec;
use graphviz_rust::printer::PrinterContext;
use uuid::Uuid;

#[derive(Clone)]
pub struct Value {
    pub id: Uuid,
    pub data: f64,
    pub prev: Vec<Rc<RefCell<Value>>>,
    pub op: String,
    pub label: String,
    pub grad: f64,
}

#[derive(Clone)]
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
        write!(f, "Value(data={},grad={})", self.data, self.grad)
    }
}

impl fmt::Debug for NNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value(data={})", self.root.as_ref().borrow().data,)
    }
}

pub fn back(root: Rc<RefCell<Value>>) {
    let bind = root.as_ref().borrow_mut();
    let l = bind.prev.len();
    let op = &bind.op;
    let grad = bind.grad;
    if l != 0 {
        match op.as_str() {
            "+" => {
                bind.prev
                    .iter()
                    .for_each(|item| item.as_ref().borrow_mut().grad += 1.0 * grad);
            }
            "-" => {
                bind.prev
                    .iter()
                    .for_each(|item| item.as_ref().borrow_mut().grad += 1.0 * grad);
            }
            "*" => {
                let item1 = &mut bind.prev[0].as_ref().borrow_mut();
                let item2 = &mut bind.prev[1].as_ref().borrow_mut();

                item1.grad += item2.data * grad;
                item2.grad += item1.data * grad;
            }
            "tanh" => {
                let item1 = &mut bind.prev[0].as_ref().borrow_mut();
                item1.grad += (1.0 - bind.data.powf(2.0)) * grad;
            }
            "^" => {
                let item1 = &mut bind.prev[0].as_ref().borrow_mut();
                let item2 = &mut bind.prev[1].as_ref().borrow_mut();

                item1.grad += item2.data * item1.data.powf(item2.data - 1.0);
                item2.grad += 0.0;
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
pub fn add_vec(values: &Vec<NNode>) -> NNode {
    let out: NNode = NNode {
        root: Rc::new(RefCell::new(Value {
            id: Uuid::new_v4(),
            data: values.iter().map(|v| v.root.as_ref().borrow().data).sum(),
            prev: values.into_iter().map(|v| Rc::clone(&v.root)).collect(),
            op: "+".to_string(),
            label: "".to_string(),
            grad: 0.0,
        })),
    };
    out
}

impl NNode {
    pub fn new(data: f64) -> Self {
        Self {
            root: Rc::new(RefCell::new(Value {
                id: Uuid::new_v4(),
                data,
                prev: Vec::new(),
                op: "".to_string(),
                label: "".to_string(),
                grad: 0.0,
            })),
        }
    }

    pub fn init(data: f64, label: String) -> Self {
        Self {
            root: Rc::new(RefCell::new(Value {
                id: Uuid::new_v4(),
                data,
                prev: Vec::new(),
                op: "".to_string(),
                label,
                grad: 0.0,
            })),
        }
    }

    fn from(self, nnode: &NNode) -> Self {
        Self {
            root: Rc::clone(&nnode.root),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            root: Rc::new(RefCell::new(Value {
                id: Uuid::new_v4(),
                data: self.root.as_ref().borrow().data + other.root.as_ref().borrow().data,
                prev: Vec::from([Rc::clone(&self.root), Rc::clone(&other.root)]),
                op: "+".to_string(),
                label: "".to_string(),
                grad: 0.0,
            })),
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            root: Rc::new(RefCell::new(Value {
                id: Uuid::new_v4(),
                data: self.root.as_ref().borrow().data - other.root.as_ref().borrow().data,
                prev: Vec::from([Rc::clone(&self.root), Rc::clone(&other.root)]),
                op: "-".to_string(),
                label: "".to_string(),
                grad: 0.0,
            })),
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self {
            root: Rc::new(RefCell::new(Value {
                id: Uuid::new_v4(),
                data: self.root.as_ref().borrow().data * other.root.as_ref().borrow().data,
                prev: Vec::from([Rc::clone(&self.root), Rc::clone(&other.root)]),
                op: "*".to_string(),
                label: "".to_string(),
                grad: 0.0,
            })),
        }
    }

    pub fn pow(&self, other: &Self) -> Self {
        Self {
            root: Rc::new(RefCell::new(Value {
                id: Uuid::new_v4(),
                data: self
                    .root
                    .as_ref()
                    .borrow()
                    .data
                    .powf(other.root.as_ref().borrow().data),
                prev: Vec::from([Rc::clone(&self.root), Rc::clone(&other.root)]),
                op: "^".to_string(),
                label: "".to_string(),
                grad: 0.0,
            })),
        }
    }

    pub fn tanh(&self) -> Self {
        let data = self.root.as_ref().borrow().data;
        Self {
            root: Rc::new(RefCell::new(Value {
                id: Uuid::new_v4(),
                data: (E.powf(2.0 * data) - 1.0) / (E.powf(2.0 * data) + 1.0),
                prev: Vec::from([Rc::clone(&self.root)]),
                op: "tanh".to_string(),
                label: "".to_string(),
                grad: 0.0,
            })),
        }
    }

    pub fn visualise(&self) {
        let root = &self.clone().root;
        let (nodes, edges) = (trace(root.clone()).0, trace(root.clone()).1);

        let mut dot: Graph = graph!(strict di id!("micro"); attr!("rankdir", "LR"));

        for item in nodes.iter() {
            let n = item.as_ref().borrow();
            let uid: Id = id!(n.id.to_string());
            let text: String = String::from(
                n.label.to_string()
                    + "| Data "
                    + &n.data.to_string()
                    + "| Grad "
                    + &n.grad.to_string(),
            );
            dot.add_stmt(stmt!(
                node!(esc uid;  attr!(esc "label",esc &text), attr!("shape", "record"))
            ));

            if n.op != "".to_string() {
                let temp_uid: String = n.id.to_string() + &n.op.to_string();
                dot.add_stmt(stmt!(node!(esc temp_uid;attr!("label",esc n.op))));
                dot.add_stmt(stmt!(edge!(node_id!(esc temp_uid) => node_id!(esc uid))))
            }

            for (val1, val2) in edges.iter() {
                let n1 = val1.as_ref().borrow();
                let n2 = val2.as_ref().borrow();
                dot.add_stmt(stmt!(edge!(node_id!(esc n1.id.to_string()) => node_id!(esc n2.id.to_string() + &n2.op.to_string()))));
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

        // self.root.as_ref().borrow_mut().grad = 1.0;

        for v in topo {
            back(v);
        }
        // println!("{:?}", &topo);
    }
}
