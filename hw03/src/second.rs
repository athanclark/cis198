#[derive(Debug)]
pub struct BST<T> {
    root: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: BST<T>,
    right: BST<T>,
}

impl<T> Node<T> {
    pub fn map<U>(self, f: fn(T) -> U) -> Node<U> {
        Node {
            elem: f(self.elem),
            left: self.left.map(f),
            right: self.right.map(f),
        }
    }
    pub fn as_ref(&self) -> Node<&T> {
        Node {
            elem: &self.elem,
            left: self.left.as_ref(),
            right: self.right.as_ref(),
        }
    }
    pub fn as_mut(&mut self) -> Node<&mut T> {
        Node {
            elem: &mut self.elem,
            left: self.left.as_mut(),
            right: self.right.as_mut(),
        }
    }
}

impl<T> BST<T> {
    pub fn new() -> BST<T> {
        BST{ root: None }
    }
    pub fn map<U>(self, f: fn(T) -> U) -> BST<U> {
        BST{ root: self.root.map(|node| Box::new((*node).map(f))) }
    }
    pub fn as_ref(&self) -> BST<&T> {
        BST{ root: self.root.as_ref().map(|node| Box::new((**node).as_ref())) }
    }
    pub fn as_mut(&mut self) -> BST<&mut T> {
        BST{ root: self.root.as_mut().map(|node| Box::new((**node).as_mut())) }
    }

    pub fn take_left(&mut self) -> Option<T> {
        match self.root.take() {
            None => None,
            Some(mut boxed_node) => {
                if boxed_node.left.root.is_some() {
                    let result = boxed_node.left.take_left();
                    self.root = Some(boxed_node);
                    result
                } else {
                    self.root = boxed_node.right.root;
                    Some(boxed_node.elem)
                }
            }
        }
    }

    pub fn take_right(&mut self) -> Option<T> {
        match self.root.take() {
            None => None,
            Some(mut boxed_node) => {
                if boxed_node.right.root.is_some() {
                    let result = boxed_node.right.take_right();
                    self.root = Some(boxed_node);
                    result
                } else {
                    self.root = boxed_node.left.root;
                    Some(boxed_node.elem)
                }
            }
        }
    }

    pub fn peek_left(&self) -> Option<&T> {
        self.root.as_ref().and_then(|node| {
            if node.left.root.is_some() {
                node.left.peek_left()
            } else {
                Some(&node.elem)
            }
        })
    }

    pub fn peek_right(&self) -> Option<&T> {
        self.root.as_ref().and_then(|node| {
            if node.right.root.is_some() {
                node.right.peek_right()
            } else {
                Some(&node.elem)
            }
        })
    }

    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.root.as_mut().and_then(|node| {
            if node.left.root.is_some() {
                node.left.peek_left_mut()
            } else {
                Some(&mut node.elem)
            }
        })
    }

    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.root.as_mut().and_then(|node| {
            if node.right.root.is_some() {
                node.right.peek_right_mut()
            } else {
                Some(&mut node.elem)
            }
        })
    }
}

impl<T: PartialOrd> BST<T> {
    pub fn insert(&mut self, x: T) -> bool {
        match self.root.take() {
            None => {
                self.root = Some(Box::new(Node{
                    elem: x,
                    left: BST{ root: None },
                    right: BST{ root: None },
                }));
                true
            }
            Some(mut boxed_node) => {
                if boxed_node.elem == x {
                    false
                } else {
                    let result: bool =
                        if x < boxed_node.elem {
                            boxed_node.left.insert(x)
                        } else {
                            boxed_node.right.insert(x)
                        };
                    self.root = Some(boxed_node);
                    result
                }
            }
        }
    }

    pub fn search(&self, x: T) -> bool {
        match self.root {
            None => false,
            Some(ref boxed_node) => {
                if boxed_node.elem == x {
                    true
                } else {
                    if x < boxed_node.elem {
                        boxed_node.left.search(x)
                    } else {
                        boxed_node.right.search(x)
                    }
                }
            }
        }
    }
}

// owned iterator
pub struct IntoIter<T>(BST<T>);
impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take_left()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.take_right()
    }
}

// borrowed iterator
pub struct Iter<'a, T>(BST<&'a T>);
impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self.as_ref())
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take_left()
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.take_right()
    }
}

// mutable iterator
pub struct IterMut<'a, T>(BST<&'a mut T>);
impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.as_mut())
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take_left()
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.take_right()
    }
}


#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn tests() {
        let mut tree: BST<i32> = BST::new();

        let added1 = tree.insert(1);

        assert!(added1);

        let added2 = tree.insert(2);

        assert!(added2);

        let added4 = tree.insert(4);

        assert!(added4);

        assert!(tree.search(1));
        assert!(tree.search(2));
        assert!(tree.search(4));
        assert!(!tree.search(3));
    }

    #[test]
    fn into_iter() {
        let mut tree: BST<i32> = BST::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(0);
        tree.insert(3);

        let mut iter = tree.into_iter();

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(2));
    }

    #[test]
    fn iter() {
        let mut tree: BST<i32> = BST::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(0);
        tree.insert(3);

        let mut iter1 = (&tree).into_iter();
        let mut iter2 = (&tree).into_iter();
        let mut vec1: Vec<&i32> = vec![];
        let mut vec2: Vec<&i32> = vec![];

        for i in iter1 {
            vec1.push(&i);
        }

        assert_eq!(vec1, vec![&0i32,&1,&2,&3]);

        for i in iter2.rev() {
            vec2.push(&i);
        }

        assert_eq!(vec2, vec![&3i32,&2,&1,&0]);
    }

    #[test]
    fn iter_mut() {
        let mut tree: BST<i32> = BST::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(0);
        tree.insert(3);

        let mut iter1 = (&mut tree).into_iter();

        assert_eq!(iter1.next(),Some(&mut 0));
        assert_eq!(iter1.next_back(),Some(&mut 3));
        assert_eq!(iter1.next(),Some(&mut 1));
        assert_eq!(iter1.next_back(),Some(&mut 2));
        assert_eq!(iter1.next(),None);

    }
}
