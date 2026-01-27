// Binary search

fn main() {
    let mut v = vec![2, 5, 7, 34, -3, 12, 0, 9, 24];
    let item_to_search = 12;

    v.sort();
    println!("Sorted array: {:?}", v);

    let result = search(item_to_search, v);
    match result {
        Some(index) => println!("Item found at index {}", index),
        None => println!("Item not found")
    }
}

fn search(item_to_search: i64, v: Vec<i64>) -> Option<usize> {
    let mut low = 0;
    let mut high = v.len() - 1;
    
    while low <= high {
        let mid_index = (low + high) / 2;
        let guess = v[mid_index];
        if guess == item_to_search {
            return Some(mid_index);
        } else if guess < item_to_search {
            low = mid_index + 1;
        } else {
            high = mid_index - 1;
        }
    }
    return None;
}