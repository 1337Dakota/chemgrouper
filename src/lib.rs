use std::{collections::HashMap, fmt::Display};

use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::Bfs,
    Direction::{Incoming, Outgoing},
};
use tinyjson::JsonValue;

pub type ChemGraph = DiGraph<Chemical, u32>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Chemical {
    Base(String),
    Complex {
        name: String,
        deps: Vec<(Chemical, u32)>,
        reaction_temp: Option<u32>,
    },
    Maybe(String), // Fuck parsing JSON
}

impl Display for Chemical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

const FUCKERY_PANIC_MSG: &str = "WHAT THE FUCK ARE YOU DOING???";

impl Chemical {
    pub fn name(&self) -> String {
        match self {
            Chemical::Base(name) => name.clone(),
            Chemical::Complex { name, .. } => name.clone(),
            Chemical::Maybe(name) => name.clone(),
        }
    }
    pub fn deps(&self) -> Option<Vec<(Chemical, u32)>> {
        match self {
            Chemical::Base(_) => None,
            Chemical::Complex { deps, .. } => Some(deps.clone()),
            Chemical::Maybe(_) => panic!("{}", FUCKERY_PANIC_MSG),
        }
    }
    pub fn set_deps(&mut self, new: Vec<(Chemical, u32)>) {
        match self {
            Chemical::Base(_) => panic!("{}", FUCKERY_PANIC_MSG),
            Chemical::Complex { deps, .. } => *deps = new,
            Chemical::Maybe(_) => panic!("{}", FUCKERY_PANIC_MSG),
        }
    }
    pub fn reaction_temp(&self) -> Option<u32> {
        match self {
            Chemical::Base(_) => None,
            Chemical::Complex { reaction_temp, .. } => *reaction_temp,
            Chemical::Maybe(_) => panic!("{}", FUCKERY_PANIC_MSG),
        }
    }
    pub fn is_base(&self) -> bool {
        match self {
            Chemical::Base(_) => true,
            Chemical::Complex { .. } => false,
            Chemical::Maybe(_) => panic!("{}", FUCKERY_PANIC_MSG),
        }
    }
    pub fn is_maybe(&self) -> bool {
        match self {
            Chemical::Base(_) => false,
            Chemical::Complex { .. } => false,
            Chemical::Maybe(_) => true,
        }
    }
}

pub fn build_graph(target: Chemical) -> ChemGraph {
    fn add_deps(deps: Vec<(Chemical, u32)>, parent: NodeIndex, graph: &mut ChemGraph) {
        for (dep, amount) in deps {
            let dep_node = graph.add_node(dep);
            graph.add_edge(parent, dep_node, amount);
        }
    }

    let mut graph = ChemGraph::new();

    let deps = target
        .deps()
        .expect("Don't build a graph for a Base Chemical");

    let root = graph.add_node(target);

    add_deps(deps, root, &mut graph);

    while graph
        .externals(Outgoing)
        .any(|v| !graph.node_weight(v).unwrap().is_base())
    {
        let complex_nodes: Vec<_> = graph
            .externals(Outgoing)
            .filter(|v| !graph.node_weight(*v).unwrap().is_base())
            .collect();

        for node in complex_nodes {
            let weight = graph.node_weight(node).unwrap();
            add_deps(weight.deps().unwrap(), node, &mut graph);
        }
    }

    graph
}

pub fn build_steps(graph: ChemGraph) -> String {
    let root = graph.externals(Incoming).next().unwrap();
    let mut bfs = Bfs::new(&graph, root);
    let mut result = Vec::new();
    let mut distances = HashMap::new();
    distances.insert(root, 0);

    while let Some(node) = bfs.next(&graph) {
        let current_distance = distances[&node];
        let sep = " ".repeat(current_distance * 4);
        let chemical = graph.node_weight(node).unwrap();
        for neighbor in graph.neighbors_directed(node, Outgoing) {
            // If neighbor hasn't been visited, calculate its distance
            distances
                .entry(neighbor)
                .or_insert_with(|| current_distance + 1);
        }

        let name = chemical.name();
        let parts = {
            if node == root {
                1u32
            } else {
                let parent = graph.neighbors_directed(node, Incoming).next().unwrap();
                let edge = graph.find_edge(parent, node).unwrap();
                *graph.edge_weight(edge).unwrap()
            }
        };

        result.push(format!("{sep}{parts} parts of {name}"));
    }

    result.join("\n")
}

// Don't look at this function
// Look at the others, they're beautiful
// This one is not
// Ignore it
// Dont refactor it
// It's not worth the mental pain
// It works and that's all that matters
pub fn parse_json(json: &str) -> Vec<Chemical> {
    let mut result = Vec::new();
    let parsed: JsonValue = json.parse().unwrap();
    let array: &Vec<_> = parsed.get().unwrap();

    for chemical in array {
        let chemical: &HashMap<_, _> = chemical.get().unwrap();
        let name: &String = chemical.get("name").unwrap().get().unwrap();
        let deps: &HashMap<String, JsonValue> = chemical.get("deps").unwrap().get().unwrap();
        let deps: HashMap<&String, u32> = deps
            .iter()
            .map(|(k, v)| (k, *v.get::<f64>().unwrap() as u32))
            .collect();
        let deps: Vec<(Chemical, u32)> = deps
            .iter()
            .map(|(key, &value)| (Chemical::Maybe(key.to_string()), value))
            .collect();
        let reaction_temp: Option<&f64> = chemical.get("reaction_temp").unwrap().get();
        let reaction_temp = reaction_temp.map(|v| *v as u32);
        let obj = Chemical::Complex {
            name: name.to_string(),
            deps,
            reaction_temp,
        };

        result.push(obj);
    }

    while let Some(parent_pos) = result.iter().position(|v| {
        v.deps()
            .is_some_and(|v2| v2.iter().any(|(c, _)| c.is_maybe()))
    }) {
        let mut parent = result[parent_pos].clone();
        let deps = parent.deps().unwrap();
        let mut new_deps = deps.clone();
        let offenders: Vec<&(Chemical, u32)> = deps.iter().filter(|(c, _)| c.is_maybe()).collect();

        for offender in offenders {
            let dep_pos = deps
                .iter()
                .position(|(c, _)| c.name() == offender.0.name())
                .unwrap();
            if let Some(target) = result.iter().find(|v| v.name() == offender.0.name()) {
                new_deps[dep_pos].0 = target.clone();
            } else {
                new_deps[dep_pos].0 = Chemical::Base(offender.0.name());
            }
        }
        parent.set_deps(new_deps);
        result[parent_pos] = parent;
    }

    result
}