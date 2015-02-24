extern crate bsort;

use bsort::bsort::BSortable;

fn main() {
    let sorted_items = {
        let mut unsorted_items: Vec<usize> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
        unsorted_items.bsort();
        unsorted_items
    };

    let rev_items = {
        let mut unsorted_items: Vec<usize> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
        unsorted_items.bsort_by(|&t1, t2| t2.partial_cmp(&t1).unwrap());
        unsorted_items
    };

    for (a,b) in sorted_items.iter().zip(rev_items.iter()) {
        println!("{}\t{}", a, b);
    }
}
