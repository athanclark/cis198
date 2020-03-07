use std::mem;



#[derive(Debug)]
pub struct BST {
    root: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl BST {
    pub fn new() -> BST {
        BST{ root: Link::Empty }
    }

    pub fn insert(&mut self, x: i32) -> bool {
        self.root.insert(x)
    }

    pub fn search(&self, x: i32) -> bool {
        self.root.search(x)
    }
}

impl Link {
    pub fn insert(&mut self, x: i32) -> bool {
        match mem::replace(self, Link::Empty) {
            Link::Empty => {
                *self = Link::More(Box::new(Node{
                    elem: x,
                    left: Link::Empty,
                    right: Link::Empty,
                }));
                true
            },
            Link::More(mut boxed_node) => {
                if boxed_node.elem == x {
                    false
                } else {
                    let result: bool =
                        if x < boxed_node.elem {
                            boxed_node.left.insert(x)
                        } else {
                            boxed_node.right.insert(x)
                        };
                    *self = Link::More(boxed_node);
                    result
                }
            },
        }
    }

    pub fn search(&self, x: i32) -> bool {
        match self {
            Link::Empty => false,
            Link::More(ref boxed_node) => {
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


#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn tests() {
        let mut tree: BST = BST::new();

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
}
