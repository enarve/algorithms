// Depth-first search

use std::collections::{HashMap, HashSet};

fn main() {
    let mut graph = HashMap::new();
    prepare_graph(&mut graph);
    let origin = "home";
    let destination = "torzhok";
    let mut searched: HashSet<&str> = HashSet::new();
    let result = search_path(&graph, origin, destination, 1, &mut searched);
    match result {
        Some(length) => println!("{} stops from {} to {}", length, origin, destination),
        None => println!("Can't reach {} from {}!", destination, origin)
    }
}

fn search_path<'a>(graph: &HashMap<&str, Vec<&'a str>>, origin: &str, destination: &str, iter_number: i32, searched: &mut HashSet<&'a str>) -> Option<i32> {
    let out_neighbors = graph[origin].clone().into_iter().map(|edge| (edge, iter_number));
    for x in out_neighbors {
        if !(searched.contains(x.0)) {
            if x.0 == destination {
                println!("Found {}", x.0);
                return Some(x.1)
            } else {
                println!("Searched {}", x.0);
                searched.insert(x.0);
                match search_path(&graph, x.0, destination, iter_number + 1, searched) {
                    Some(length) => return Some(length),
                    None => ()
                }
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