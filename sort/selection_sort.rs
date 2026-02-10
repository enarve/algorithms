// Selection sort

fn main() {
    let v = vec![8, 4, 9, 12, 1, 47, 3, -6, 15, 0];
    println!("{:?}", v);
    let sorted = selection_sort(&v);
    println!("{:?} — sorted", sorted);
}

fn selection_sort(v: &Vec<i32>) -> Vec<i32> {
    let mut sorted = vec![];
    let mut unsorted = v.clone();
    while unsorted.len() > 0 {
        let mut min_index = 0;
        for (i, x) in unsorted.clone().into_iter().enumerate() {
            if x < unsorted[min_index] {
                min_index = i;
            }
        }
        sorted.push(unsorted.remove(min_index));
    }
    return sorted;
}