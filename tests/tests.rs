#[cfg(test)]
mod tests {
    use chemgrouper::{build_graph, build_steps, parse_json};
    use petgraph::Direction::{Incoming, Outgoing};

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

    #[test]
    fn test_hydroxide_complex() {
        let json = include_str!("../out.json");
        let chemicals = parse_json(json);
        let hydroxide = chemicals
            .iter()
            .find(|c| c.name() == *"Hydroxide")
            .unwrap()
            .clone();

        assert!(!hydroxide.is_maybe());
        assert!(!hydroxide.is_base());
    }

    #[test]
    fn test_phenol_hydroxide_graph() {
        let json = include_str!("../out.json");
        let chemicals = parse_json(json);
        let phenol = chemicals
            .iter()
            .find(|c| c.name() == *"Phenol")
            .unwrap()
            .clone();
        let graph = build_graph(phenol);
        let phenol_node = graph.externals(Incoming).next().unwrap();
        eprintln!("{:#?}", graph);
        let deps: Vec<_> = graph.neighbors_directed(phenol_node, Outgoing).collect();
        let hydroxide_node = deps
            .iter()
            .find(|node| graph.node_weight(**node).unwrap().name() == *"Hydroxide")
            .unwrap();
        let hydroxide = graph.node_weight(*hydroxide_node).unwrap();
        eprintln!("{:#?}", hydroxide);
        assert!(!hydroxide.is_base());
    }
}
