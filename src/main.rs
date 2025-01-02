mod binary_heap;

use binary_heap::{BinaryHeap, BinaryHeapType};

fn main() {
    let mut tmp = BinaryHeap::new(BinaryHeapType::Max);
    tmp.push(1);
    tmp.push(5);
    tmp.push(3);
    tmp.display();

    let vec = [
        10, 20, 5, 3, 6, 16, 11, 25, 13, 19, 2, 30, 10, 50, 30, 20, 33, 12, 45, 23, 43, 23, 68, 65,
        12, 22, 6, 7, 8, 9, 10, 101, 102, 103, 104, 105, 106, 107, 108, 101, 102, 103, 104, 105,
        106, 107, 108, 101, 102, 103, 104, 105, 106, 107, 108, 101, 102, 103, 104, 105,
        106, //1074,
    ];
    let mut tree = BinaryHeap::hipify(BinaryHeapType::Min, vec.to_vec());
    tree.push(1084);
    tree.display();

    println!("Sorted:");
    while let Some(v) = tree.pop() {
        print!("{:?}, ", v);
    }
    println!();
}
