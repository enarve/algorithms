// Simple search

fn main() {
    let v = vec![2, 5, 7, 34, -3, 12, 0, 9, 24];
    let item_to_search = 12;

    let result = search(item_to_search, v);
    match result {
        Some(index) => println!("Item found at index {}", index),
        None => println!("Item not found")
    }
}

fn search(item_to_search: i64, v: Vec<i64>) -> Option<usize> {
    for (index, item) in v.into_iter().enumerate() {
        if item == item_to_search {
            return Some(index);
        }
    }
    return None;
}