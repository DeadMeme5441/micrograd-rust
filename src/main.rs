mod engine;
use engine::Value;
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::exec;
use graphviz_rust::exec_dot;
use graphviz_rust::printer::{DotPrinter, PrinterContext};

fn main() {
    let a: Value = Value::init(2.0, "a".to_string());
    let b: Value = Value::init(-3.0, "b".to_string());
    let c: Value = Value::init(10.0, "c".to_string());

    let mut e = a * b;
    e.label = "e".to_string();

    let mut d = e + c;
    d.label = "d".to_string();

    let f = Value::init(-2.0, "f".to_string());

    let mut L = d * f;

    L.label = "L".to_string();

    draw_dot(&L);
}

fn trace(root: &Value) -> (Vec<Value>, Vec<(Value, Value)>) {
    let mut nodes: Vec<Value> = Vec::new();
    let mut edges: Vec<(Value, Value)> = Vec::new();

    build(root, &mut nodes, &mut edges);

    (nodes, edges)
}

fn build(v: &Value, nodes: &mut Vec<Value>, edges: &mut Vec<(Value, Value)>) {
    if !nodes.contains(&v) {
        nodes.push(v.clone());
    }
    for child in v.prev.iter() {
        edges.push((child.clone(), v.clone()));

        build(child, nodes, edges);
    }
}

fn draw_dot(root: &Value) {
    let (nodes, edges) = (trace(root).0, trace(root).1);

    let mut dot: Graph = graph!(strict di id!("micro"); attr!("rankdir", "LR"));

    for n in nodes.iter() {
        let uid: Id = id!(n.label);
        let text: String = String::from(n.label.to_string() + "|" + &n.data.to_string());
        dot.add_stmt(stmt!(
            node!(uid;  attr!("label",esc &text), attr!("shape", "record"))
        ));

        if n.op != "".to_string() {
            let temp_uid: Id = id!(n.label.to_string() + &n.op.to_string());
            dot.add_stmt(stmt!(node!(esc temp_uid;attr!("label",esc n.op))));
            dot.add_stmt(stmt!(edge!(node_id!(esc temp_uid) => node_id!(esc uid))))
        }

        for (n1, n2) in edges.iter() {
            dot.add_stmt(stmt!(edge!(node_id!(esc n1.label.to_string()) => node_id!(esc n2.label.to_string() + &n2.op.to_string()))));
        }
    }

    let mut ctx = PrinterContext::default();
    let empty = exec(
        dot,
        &mut ctx,
        vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output("output.svg".to_string()),
        ],
    );
}
