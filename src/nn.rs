use rand::Rng;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;

use crate::engine::*;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub w: Vec<NNode>,
    pub b: NNode,
}

#[derive(Debug, Clone)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug, Clone)]
pub struct MLP {
    size: Vec<usize>,
    layers: Vec<Layer>,
}

impl From<usize> for Neuron {
    fn from(nin: usize) -> Self {
        Self {
            w: (0..nin)
                .map(|_| NNode::new(rand::thread_rng().gen_range(-1.0..=1.0)))
                .collect(),
            b: NNode::init(rand::thread_rng().gen_range(-1.0..=1.0), "b".to_string()),
        }
    }
}

impl Neuron {
    pub fn call(&self, x: &Vec<NNode>) -> NNode {
        let mut act: Vec<NNode> = zip(&self.w, x)
            .map(|(w, x)| w.mul(&x))
            .collect::<Vec<NNode>>();

        act.push(self.b.clone());

        let out = add_vec(&act);
        let norm = NNode::from(out.tanh());
        norm
    }

    pub fn parameters(&self) -> Vec<NNode> {
        let mut out: Vec<NNode> = self.w.iter().map(|x| x.clone()).collect();
        out.push(self.b.clone());

        out
    }
}

impl Layer {
    pub fn new(nin: usize, nout: usize) -> Self {
        Self {
            neurons: (0..nout).map(|_| Neuron::from(nin)).collect(),
        }
    }

    pub fn call(&self, x: &Vec<NNode>) -> Vec<NNode> {
        let mut out: Vec<NNode> = Vec::new();
        for n in &self.neurons {
            out.push(n.call(x));
        }

        out
    }

    pub fn parameters(&self) -> Vec<NNode> {
        let mut out: Vec<NNode> = Vec::new();

        for n in &self.neurons {
            let mut ps = n.parameters();
            out.append(&mut ps);
        }

        out
    }
}

impl MLP {
    pub fn new(nin: usize, nouts: &mut Vec<usize>) -> Self {
        let mut sz: Vec<usize> = [nin].to_vec();
        sz.append(nouts);
        let mut lay: Vec<Layer> = Vec::new();

        for i in 0..sz.len() - 1 {
            lay.push(Layer::new(sz[i], sz[i + 1]));
        }

        Self {
            size: sz,
            layers: lay,
        }
    }

    pub fn call(&self, x: &Vec<NNode>) -> Vec<NNode> {
        let mut out: Vec<NNode> = x.into_iter().map(|x| NNode::from(x.clone())).collect();

        for l in &self.layers {
            out = l.call(&out);
        }

        out
    }

    pub fn parameters(&self) -> Vec<NNode> {
        let mut out = Vec::new();
        for l in &self.layers {
            let mut ls = l.parameters();
            out.append(&mut ls);
        }

        out
    }
}
