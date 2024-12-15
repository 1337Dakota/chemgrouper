#[cfg(test)]
mod tests {
    use chemgrouper::{build_graph, build_steps, parse_json};

    #[test]
    fn test_basic_shit() {
        let json = include_str!("../test.json");
        let chemicals = parse_json(json);
        assert!(!chemicals
            .iter()
            .any(|c| c.deps().unwrap().iter().any(|(c, _)| c.is_maybe())));
    }

    #[test]
    fn test_all() {
        let json = include_str!("../out.json");
        let chemicals = parse_json(json);
        for chemical in chemicals {
            eprintln!("Testing {:#?}", chemical);
            let graph = build_graph(chemical);
            let steps = build_steps(graph);
            assert!(!steps.is_empty())
        }
    }
}
