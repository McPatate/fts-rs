use rand::Rng;
use std::fmt;
use std::iter;
use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

/// A SkipList with owned nodes
///
/// The `SkipList` allows search, insertion and deletion
/// in *O*(*log*(*n*)) time.
/// It keeps its members in sorted order.
pub struct SkipList<T> {
    head: Link<T>,
    k_max_height: u16,
    max_height: u16,
    branching_factor: u16,
    inverse_branching: f64,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
    element: T,
    next: Vec<Link<T>>,
}

#[derive(Clone)]
pub struct Iter<'a, T> {
    head: Link<T>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

// #[derive(Clone)]
pub struct IntoIter<T> {
    list: SkipList<T>,
}

impl<T: Default> Node<T> {
    fn new(elt: T, height: u16) -> Self {
        Node {
            element: elt,
            next: iter::repeat(None).take(height.into()).collect(),
        }
    }

    pub fn into_element(self: Box<Self>) -> T {
        self.element
    }

    fn new_head(height: u16) -> Link<T> {
        // any value will do
        let node = Box::new(Self::new(Default::default(), height));
        Some(Box::leak(node).into())
    }

    //    fn key(&self) -> Option<&T> {
    //        self.key.as_ref()
    //    }
    //
    //    fn next(&self, height: u16) -> LinkRef<T> {
    //        assert!(height >= 0);
    //        self.next[height as usize].as_ref()
    //    }
    //
    //    fn set_next(&mut self, height: u16, n: Link<T>) -> () {
    //        assert!(height >= 0);
    //        self.next[height as usize] = n;
    //    }
}

// impl<T: Clone> Clone for Node<T> {
//     fn clone(&self) -> Node<T> {
//         Node {
//             key: self.key.clone(),
//             next: self.next.clone(),
//         }
//     }
// }

// private methods
impl<T> SkipList<T>
where
    T: Ord + Default,
{
    fn random_height(&self) -> u16 {
        let mut lvl = 1;
        while lvl < self.k_max_height && rand::thread_rng().gen::<f64>() < self.inverse_branching {
            lvl += 1;
        }
        lvl
    }

    // node must be initialised with heght = 0
    fn insert_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            let mut update: Vec<Link<T>> = iter::repeat(None)
                .take(self.k_max_height as usize)
                .collect();
            let mut x = self.head;
            for i in (0..self.max_height).rev() {
                loop {
                    assert!(x.is_some());
                    let x_node = x.unwrap().as_ptr();
                    let x_next = (*x_node).next[i as usize];
                    if x_next.is_none() {
                        break;
                    } else {
                        let x_next_node = x_next.unwrap().as_ptr();
                        if node.element < (*x_next_node).element {
                            // cannot insert duplicates
                            assert!(!(node.element == (*x_next_node).element));
                            break;
                        }
                    }
                    x = x_next;
                }
                update[i as usize] = x;
            }
            // Here we will know if elt is in list or not as we keep values in a HashMap
            let height = self.random_height();
            if height > self.max_height {
                for i in self.max_height..height {
                    update[i as usize] = self.head;
                }
                self.max_height = height;
            }
            for i in 0..height {
                let u = update[i as usize].unwrap().as_ptr();
                node.next.push((*u).next[i as usize]);
            }
            let node = Some(Box::leak(node).into());
            for i in 0..height {
                let u = update[i as usize].unwrap().as_ptr();
                (*u).next[i as usize] = node;
            }
            self.len += 1;
        }
    }
}

impl<T> Default for SkipList<T>
where
    T: Ord + Default,
{
    /// Creates an empty `SkipList<T>`
    #[inline]
    fn default() -> Self {
        // supposed to be good up to 2^16 elements
        Self::new(16, 4)
    }
}

impl<T> SkipList<T>
where
    T: Ord + Default,
{
    pub fn contains() {}
    pub fn new(max_height: u16, branching_factor: u16) -> SkipList<T> {
        SkipList {
            max_height: 1,
            k_max_height: max_height,
            branching_factor: branching_factor,
            head: Node::new_head(max_height),
            inverse_branching: 1.0 / branching_factor as f64,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn insert(&mut self, elt: T) {
        self.insert_node(Box::new(Node::new(elt, 0)));
    }

    // fn find_greater_or_equal(k: &T) -> Node<T> {}

    //     fn find_less_than(&self, k: &T) -> Node<T> {
    //         let x = self.head;
    //         let mut lvl = self.get_max_height();
    //         let last_not_after = None;
    //         while x.is_some() {
    //             let next = x.unwrap();
    //         }
    //         Node::new(T::default())
    //     }

    // fn find_last() -> Node<T> {}

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                // head should never be none
                assert!(self.head.is_some());
                let head_node = self.head.unwrap().as_ptr();
                self.head = (*head_node).next[0];
                // this won't panic because len > 0
                let head_node = self.head.unwrap().as_ptr();
                self.len -= 1;
                Some(&(*head_node).element)
            }
        }
    }
}

// impl<T> IntoIterator for SkipList<T> {
//     type Item = T;
//     type IntoIter = IntoIter<T>;
//
//     /// Consumes the list into an iterator yielding elements by value.
//     #[inline]
//     fn into_iter(self) -> IntoIter<T> {
//         IntoIter { list: self }
//     }
// }

// impl<'a, T> IntoIterator for &'a SkipList<T>
// where
//     T: Ord,
// {
//     type Item = &'a T;
//     type IntoIter = Iter<'a, T>;
//
//     fn into_iter(self) -> Iter<'a, T> {
//         self.iter()
//     }
// }

// impl<T: fmt::Debug> fmt::Debug for SkipList<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self).finish()
//     }
// }

// impl<T> Clone for SkipList<T>
// where
//     T: Clone,
//     T: Ord,
// {
//     fn clone(&self) -> Self {
//         self.iter().cloned().collect()
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let mut sl: SkipList<String> = SkipList::new(16, 4);
        sl.insert("wewt".to_string());
        sl.insert("blblblb".to_string());
        sl.insert("azerty".to_string());
        let mut iterator = sl.iter();
        assert_eq!(iterator.next(), Some(&"azerty".to_string()));
        assert_eq!(iterator.next(), Some(&"blblblb".to_string()));
        assert_eq!(iterator.next(), Some(&"wewt".to_string()));
        assert_eq!(iterator.next(), None);
    }
}
