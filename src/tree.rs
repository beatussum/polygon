use std::cell::{Ref, RefCell};
use std::cmp::min;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Node<T>
{
    children: RefCell<Vec<Rc<Node<T>>>>,
    index: RefCell<usize>,
    parent: RefCell<Option<Weak<Node<T>>>>,
    value: RefCell<T>
}

pub struct BFSIterator<T> { unexplored: VecDeque<Rc<Node<T>>> }

impl<T> Node<T> {
    pub fn abandon(&self, child: &Rc<Self>)
    {
        let index = *child.index.borrow();

        *child.parent.borrow_mut() = None;
        self.children.borrow_mut().swap_remove(index);

        let count = self.children.borrow().len();

        if count != 0 {
            let index = min(index, count - 1);

            *self
                .children
                .borrow_mut()[index]
                .index
                .borrow_mut() = index;
        }
    }

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

        *self.index.borrow_mut() = parent.children.borrow().len();
        *self.parent.borrow_mut() = Some(Rc::downgrade(&parent));
        parent.children.borrow_mut().push(self.clone());
    }

    pub fn bfs(self: &Rc<Self>) -> Vec<Rc<Self>> { self.bfs_iter().collect() }

    pub fn bfs_iter(self: &Rc<Self>) -> BFSIterator<T>
    {
        BFSIterator::new(self)
    }

    pub fn children(&self) -> Ref<Vec<Rc<Node<T>>>> { self.children.borrow() }

    pub fn detach(self: &Rc<Self>)
    {
        self.parent().map(|parent| parent.abandon(self));
    }

    pub fn is_leaf(&self) -> bool { self.children.borrow().is_empty() }
    pub fn is_root(&self) -> bool { self.parent.borrow().is_none() }

    pub fn grandparent(&self) -> Option<Rc<Self>>
    {
        self.parent().and_then(|parent| parent.parent())
    }

    pub fn new(value: T) -> Rc<Self>
    {
        Rc::new(
            Node {
                children: RefCell::new(Vec::new()),
                index: RefCell::new(usize::default()),
                parent: RefCell::new(None),
                value: RefCell::new(value)
            }
        )
    }

    pub fn parent(&self) -> Option<Rc<Self>>
    {
        self.parent.borrow().as_ref().and_then(|parent| parent.upgrade())
    }

    pub fn set_value(&self, value: T) { *self.value.borrow_mut() = value; }

    pub fn upgrade(self: &Rc<Self>)
    {
        self
            .grandparent()
            .map(|grandparent| self.attach(&grandparent.clone()));
    }

    pub fn value(&self) -> Ref<T> { self.value.borrow() }
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

            for i in ret.children.borrow().clone() {
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
