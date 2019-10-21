mod disjoint_set;

use disjoint_set::DisjointSet;

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut ds = DisjointSet::new(&vec1);
    println!("{:#?}", ds);
    println!("Vector is: {:?}", vec1);
    println!("Parent of 1 is {}", ds.find(&1));
    println!("Components: {}", ds.get_count());
    ds.union(&1, &2);
    println!("Components: {}", ds.get_count());
    ds.union(&3, &4);
    println!("Components: {}", ds.get_count());
    ds.union(&1, &3);
    println!("Components: {}", ds.get_count());
    println!("{:?}", ds);
    println!("Parent of 4 is {}", ds.find(&4));
    println!("{:?}", ds);
    println!("Vector is still here: {:?}", vec1);
    println!("Connected (1, 4)? {}", ds.connected(&1, &4));
    println!("Connected (1, 5)? {}", ds.connected(&1, &5));
    println!("Components: {}", ds.get_count());
}
