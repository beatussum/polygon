use std::cell::{Ref, RefCell, RefMut};
use std::cmp::min;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Cell<T>
{
    children: Vec<Rc<Node<T>>>,
    index: usize,
    parent: Option<Weak<Node<T>>>,
    value: T
}

#[derive(Clone, Debug)]
pub struct Node<T>(RefCell<Cell<T>>);

pub struct BFSIterator<T> { unexplored: VecDeque<Rc<Node<T>>> }

impl<T> Node<T> {
    pub fn above(&self, n: usize)
        -> Result<Rc<Self>, (usize, Option<Rc<Self>>)>
    {
        match self.parent() {
            Some(mut parent) => {
                let mut i = n - 1;
                let mut grandparent = parent.parent();

                while grandparent.is_some() {
                    if i == 0 {
                        break;
                    } else {
                        parent = grandparent.clone().unwrap();
                        grandparent = grandparent.unwrap().parent();
                        i -= 1;
                    }
                }

                if i == 0 {
                    Ok(parent)
                } else {
                    Err((n - i, Some(parent)))
                }
            }

            None => Err((0, None))
        }
    }

    pub fn adopt(self: &Rc<Self>, child: &Rc<Self>) { child.attach(self); }

    pub fn attach(self: &Rc<Self>, parent: &Rc<Self>)
    {
        self.detach();

        self.borrow_mut().index = parent.borrow().children.len();
        self.borrow_mut().parent = Some(Rc::downgrade(&parent));
        parent.borrow_mut().children.push(self.clone());
    }

    pub fn bfs(self: &Rc<Self>) -> Vec<Rc<Self>> { self.bfs_iter().collect() }

    pub fn bfs_iter(self: &Rc<Self>) -> BFSIterator<T>
    {
        BFSIterator::new(self)
    }

    fn borrow(&self) -> Ref<Cell<T>> { self.0.borrow() }
    fn borrow_mut(&self) -> RefMut<Cell<T>> { self.0.borrow_mut() }

    pub fn children(&self) -> Ref<Vec<Rc<Node<T>>>>
    {
        Ref::map(self.borrow(), |x| &x.children)
    }

    pub fn detach(&self)
    {
        match self.parent() {
            Some(parent) => {
                self.borrow_mut().parent = None;

                parent
                    .borrow_mut()
                    .children
                    .swap_remove(self.borrow().index);

                let count = parent.borrow().children.len();

                if count != 0 {
                    let index = min(self.borrow().index, count - 1);

                    parent
                        .borrow_mut()
                        .children[index]
                        .borrow_mut()
                        .index = index;
                }
            }

            None => ()
        }
    }

    pub fn is_leaf(&self) -> bool { self.borrow().children.is_empty() }
    pub fn is_root(&self) -> bool { self.borrow().parent.is_none() }

    pub fn grandparent(&self) -> Option<Rc<Self>>
    {
        match self.parent() {
            Some(parent) => parent.parent(),
            None => None
        }
    }

    pub fn new(value: T) -> Rc<Self>
    {
        Rc::new(
            Node(
                RefCell::new(
                    Cell {
                        children: Vec::new(),
                        index: usize::default(),
                        parent: None,
                        value
                    }
                )
            )
        )
    }

    pub fn parent(&self) -> Option<Rc<Self>>
    {
        match self.borrow().parent {
            Some(ref parent) => parent.upgrade(),
            None => None
        }
    }

    pub fn set_value(&self, value: T) { self.borrow_mut().value = value; }

    pub fn upgrade(self: &Rc<Self>)
    {
        match self.grandparent() {
            Some(grandparent) => self.attach(&grandparent.clone()),
            None => ()
        }
    }

    pub fn value(&self) -> Ref<T> { Ref::map(self.borrow(), |x| &x.value) }
}

impl<T> BFSIterator<T> {
    pub fn new(node: &Rc<Node<T>>) -> Self
    {
        let mut unexplored = VecDeque::new();
        unexplored.push_back(node.clone());

        Self { unexplored }
    }
}

impl<T> Iterator for BFSIterator<T> {
    type Item = Rc<Node<T>>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.unexplored.is_empty() {
            None
        } else {
            let ret = self.unexplored.pop_front().unwrap();

            for i in ret.borrow().children.clone() {
                self.unexplored.push_back(i);
            }

            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn generate_tree() -> (Rc<Node<i32>>, Rc<Node<i32>>)
    {
        let a = Node::new(0);
        let b = Node::new(1);
        let c = Node::new(2);
        let d = Node::new(3);
        let e = Node::new(4);
        let f = Node::new(5);

        a.adopt(&b);
        a.adopt(&c);
        a.adopt(&d);
        b.adopt(&e);
        b.adopt(&f);

        (a, f)
    }

    #[test]
    fn test_above_correct()
    {
        let (a, f) = generate_tree();

        assert!(Rc::ptr_eq(&f.above(2).unwrap(), &a));
    }

    #[test]
    fn test_above_root()
    {
        let (n, ancestor) = Node::new(0).above(0).err().unwrap();

        assert_eq!(n, 0);
        assert!(ancestor.is_none());
    }

    #[test]
    fn test_above_too_much()
    {
        let (a, f) = generate_tree();

        let (n, ancestor) = f.above(5).err().unwrap();

        assert_eq!(n, 2);
        assert!(Rc::ptr_eq(&a, &ancestor.unwrap()));
    }

    #[test]
    fn test_adopt()
    {
        let a = Node::new(0);
        let b = Node::new(1);

        a.adopt(&b);

        let testing = a.children().first().unwrap().clone();

        assert_eq!(a.children().len(), 1);
        assert!(Rc::ptr_eq(&testing, &b));
        assert_eq!(*testing.value(), 1);
        assert!(Rc::ptr_eq(&testing.parent().unwrap(), &a));
    }

    #[test]
    fn test_bfs()
    {
        for (i, item) in generate_tree().0.bfs().iter().enumerate() {
            assert_eq!(*item.value(), i as i32);
        }
    }

    #[test]
    fn test_detach()
    {
        let (a, f) = generate_tree();

        f.detach();

        for (i, item) in a.bfs().iter().enumerate() {
            assert_eq!(*item.value(), i as i32);
        }

        assert!(f.is_root());
    }

    #[test]
    fn test_upgrade()
    {
        let (a, f) = generate_tree();

        f.upgrade();

        let testing =
            a
            .bfs()
            .iter()
            .map(|x| *x.value())
            .collect::<Vec<_>>();

        assert_eq!(testing, vec! [0, 1, 2, 3, 5, 4]);
        assert!(Rc::ptr_eq(&a, &f.parent().unwrap()));
    }
}
