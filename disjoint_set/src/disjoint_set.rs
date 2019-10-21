use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug)]
pub struct DisjointSet<'a, T> where T: Eq + Hash {
    parents: HashMap<&'a T, &'a T>,
    heights: HashMap<&'a T, usize>,
    count: usize,
}

impl<'a, T> DisjointSet<'a, T> where T: Eq + Hash {
    pub fn new(v: &Vec<T>) -> DisjointSet<T> {
        let parents = v.iter().map(|x| (x, x)).collect();
        let heights = v.iter().map(|x| (x, 0)).collect();
        DisjointSet { parents, heights, count: v.len() }
    }

    pub fn find(&mut self, element: &'a T) -> &'a T {
        let option = self.parents.get(element);
        match option {
            Some(&parent) => {
                let mut elem = element;
                let mut the_parent = parent;
                while elem != the_parent {
                    let grand_parent = *self.parents.get(parent).unwrap();
                    self.parents.insert(elem, grand_parent);
                    elem = grand_parent;
                    the_parent = self.parents[grand_parent];
                }
                elem
            }
            None => {
                self.parents.insert(element, element);
                self.heights.insert(element, 0);
                self.count += 1;
                element
            }
        }
    }

    pub fn union(&mut self, element_1: &'a T, element_2: &'a T) -> bool {
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

    pub fn connected(&mut self, element_1: &'a T, element_2: &'a T) -> bool {
        self.find(element_1) == self.find(element_2)
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut ds = DisjointSet::new(&vec1);
        let component_count = vec1.len();

        assert_eq!(ds.get_count(), component_count);
        assert_eq!(1, *ds.find(&1));
        assert_eq!(vec1.len(), ds.get_count());
        assert!(ds.union(&1, &2));
        assert_eq!(component_count - 1, ds.get_count());
        assert!(ds.union(&3, &4));
        assert_eq!(component_count - 2, ds.get_count());
        assert!(ds.union(&1, &3));
        assert_eq!(component_count - 3, ds.get_count());
        assert!(!vec1.is_empty());
        assert!(ds.connected(&1, &4));
        assert!(!ds.connected(&1, &5));
        assert_eq!(component_count - 3, ds.get_count());
    }
}