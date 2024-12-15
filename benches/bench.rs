use std::collections::HashMap;

use chemgrouper::{parse_json, Chemical};

fn main() {
    divan::main();
}

#[divan::bench]
fn parse_json_bench() -> HashMap<String, Vec<Chemical>> {
    parse_json(include_str!("../out.json"))
}
