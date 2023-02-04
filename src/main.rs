mod engine;
use engine::Value;
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_structures::*;
use graphviz_rust::exec;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use graphviz_rust::{self, attributes::root};
use graphviz_rust::{attributes::*, exec_dot};
use graphviz_rust::{dot_generator::*, parse};

fn main() {
    let a: Value = Value::init(2.0, "a".to_string());
    let b: Value = Value::init(-3.0, "b".to_string());
    let c: Value = Value::init(10.0, "c".to_string());

    let mut e = a * b;
    e.label = "e".to_string();

    let mut d = e + c;
    d.label = "d".to_string();

    println!("{:?}", draw_dot(&d));
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

    // let mut dot: Graph = graph!(strict di id!("graph"));

    let mut dot: Graph = Graph::DiGraph {
        id: id!("graph"),
        strict: true,
        stmts: vec![],
    };

    for n in nodes.iter() {
        let uid: Id = id!(n.label);
        let text: String = String::from(n.label.to_string() + " | " + &n.data.to_string());
        dot.add_stmt(stmt!(
            node!(uid;  attr!("label", &text), attr!("shape", "record"))
        ));

        if n.op != "".to_string() {
            let temp_uid: Id = id!(n.label.to_string() + &n.op.to_string());
            dot.add_stmt(stmt!(node!(temp_uid;attr!("label", n.op))));
            dot.add_stmt(stmt!(edge!(node_id!(temp_uid) => node_id!(uid))))
        }

        for (n1, n2) in edges.iter() {
            dot.add_stmt(stmt!(edge!(node_id!(n1.label.to_string()) => node_id!(n2.label.to_string() + &n2.op.to_string()))));
        }
    }

    let g: String = dot.print(&mut PrinterContext::default());
    println!("{:?}", g);

    let format = Format::Svg;

    let graph_svg = exec_dot(g, vec![format.clone().into()]).unwrap();

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
