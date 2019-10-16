use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug)]
struct DisjointSet<'a, T> where T: Eq + Hash {
    parents: HashMap<&'a T, &'a T>,
    heights: HashMap<&'a T, usize>,
    count: usize,
}

impl<'a, T> DisjointSet<'a, T> where T: Eq + Hash {
    fn new(v: &Vec<T>) -> DisjointSet<T> {
        let m = v.iter().map(|x| (x, x)).collect();
        let h = v.iter().map(|x| (x, 0)).collect();
        DisjointSet { parents: m, heights: h, count: v.len() }
    }

    fn find(&mut self, element: &'a T) -> &'a T {
        let option = self.parents.get(element);
        match &option {
            Some(&parent) => {
                let mut elem = element;
                let mut the_parent = parent;
                while elem != the_parent {
                    let grand_parent = *self.parents.get(parent).unwrap();
                    self.parents.insert(elem, grand_parent);
                    elem = grand_parent;
                    the_parent = self.parents[grand_parent];
                };
                elem
            }
            None => {
                self.parents.insert(element, element);
                self.heights.insert(element, 0);
                self.count += 1;
                return element;
            }
        }
    }

    fn union(&mut self, element_1: &'a T, element_2: &'a T) -> bool {
        let root_1 = self.find(element_1);
        let root_2 = self.find(element_2);

        if root_1 == root_2 {
            return false;
        }

        let height_1 = self.heights[root_1];
        let height_2 = self.heights[root_2];

        if height_1 < height_2 {
            self.parents.insert(root_1, root_2);
        } else if height_1 > height_2 {
            self.parents.insert(root_2, root_1);
        } else {
            self.parents.insert(root_2, root_1);
            self.heights.entry(root_1).and_modify(|v| *v += 1);
        }
        self.count -= 1;
        true
    }

    fn connected(&mut self, element_1: &'a T, element_2: &'a T) -> bool {
        self.find(element_1) == self.find(element_2)
    }

    fn get_count(&self) -> usize {
        self.count
    }
}

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
