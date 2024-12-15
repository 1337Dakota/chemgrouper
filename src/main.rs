use std::env::args;

use chemgrouper::{build_graph, build_steps, parse_json};

fn main() {
    let json = include_str!("../out.json");
    let versions = parse_json(json);
    let selected_version = inquire::Select::new("Select Version", versions.keys().collect())
        .prompt()
        .unwrap();
    let chemicals = versions.get(selected_version).unwrap();
    let choices: Vec<String> = chemicals.iter().map(|c| c.name()).collect();
    let arg_target = args().nth(1);
    if let Some(target) = arg_target {
        if let Some(target) = chemicals
            .iter()
            .find(|c| c.name().to_lowercase() == target.to_lowercase())
        {
            let graph = build_graph(target.clone());
            let steps = build_steps(graph);
            println!("{steps}");
            return;
        } else {
            eprintln!("Target not found");
            return;
        }
    }

    loop {
        let target = inquire::Select::new("Select Target", choices.clone())
            .prompt_skippable()
            .unwrap();
        let target = match target {
            Some(v) => chemicals.iter().find(|c| c.name() == v).unwrap().clone(),
            None => break,
        };

        let graph = build_graph(target);
        let steps = build_steps(graph);
        println!("{steps}");
    }
}
