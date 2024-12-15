use chemgrouper::{build_graph, build_steps, parse_json};

fn main() {
    let json = include_str!("../out.json");
    let chemicals = parse_json(json);
    let choices = chemicals.iter().map(|c| c.name()).collect();

    let target = inquire::Select::new("Select Target", choices)
        .prompt()
        .unwrap();
    let target = chemicals
        .iter()
        .find(|c| c.name() == target)
        .unwrap()
        .clone();

    let graph = build_graph(target);
    let steps = build_steps(graph);
    println!("{steps}");
}
