// Breadth-first search on graph

use std::collections::{HashMap, HashSet};

fn main() {
    let mut graph = HashMap::new();
    prepare_graph(&mut graph);
    let origin = "home";
    let destination = "torzhok";
    let result = search_path(&graph, origin, destination);
    match result {
        Some(length) => println!("{} stops from {} to {}", length, origin, destination),
        None => println!("Can't reach {} from {}!", destination, origin)
    }
}

fn search_path(graph: &HashMap<&str, Vec<&str>>, origin: &str, destination: &str) -> Option<i32> {
    let mut queue: Vec<(&str, i32)> = vec![];
    if origin == destination {
        return Some(0)
    }
    queue.extend(graph[origin].clone().into_iter().map(|edge| (edge, 1)));
    let mut searched: HashSet<&str> = HashSet::new();
    while !queue.is_empty() {
        let x = queue.remove(0);
        if !(searched.contains(x.0)) {
            if x.0 == destination {
                return Some(x.1)
            } else {
                searched.insert(x.0);
                queue.extend(graph[x.0].clone().into_iter().map(|edge| (edge, x.1 + 1)));
            }
        }
    }
    return None
}

fn prepare_graph(graph: &mut HashMap<&str, Vec<&str>>) {
    graph.insert("home", vec!["moscow", "nizhny"]);
    graph.insert("nizhny", vec!["kazan", "vyatka"]);
    graph.insert("kazan", vec!["vyatka"]);
    graph.insert("vyatka", vec![]);
    graph.insert("moscow", vec!["tver", "st. petersburg", "kiev"]);
    graph.insert("tver", vec!["st. petersburg", "torzhok"]);
    graph.insert("st. petersburg", vec!["helsinki", "talinn"]);
    graph.insert("helsinki", vec![]);
    graph.insert("talinn", vec!["helsinki"]);
    graph.insert("torzhok", vec![]);
    graph.insert("kiev", vec!["warshawa"]);
    graph.insert("warshawa", vec!["berlin"]);
    graph.insert("berlin", vec!["vienna"]);
    graph.insert("vienna", vec!["rim", "paris"]);
    graph.insert("paris", vec!["rim"]);
    graph.insert("rim", vec![]);
}