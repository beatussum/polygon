use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Cell<T> = RefCell<Node<T>>;

pub struct Node<T>
{
    children: Vec<Rc<Cell<T>>>,
    index: usize,
    parent: Option<Weak<Cell<T>>>,
    value: T
}

impl<T> Node<T> {
    pub fn above(&self, n: usize)
        -> Result<Rc<Cell<T>>, (usize, Option<Rc<Cell<T>>>)>
    {
        match self.parent() {
            Some(mut parent) => {
                let mut i = n;
                let mut grandparent = parent.borrow().parent();

                while grandparent.is_some() {
                    if n == 0 {
                        break;
                    } else {
                        parent = grandparent.clone().unwrap();
                        grandparent = grandparent.unwrap().borrow().parent();
                        i -= 1;
                    }
                }

                if n == 0 {
                    Ok(parent)
                } else {
                    Err((n - i, Some(parent)))
                }
            }

            None => Err((n, None))
        }
    }

    pub fn attach(&mut self, parent: Rc<Cell<T>>)
    {
        match self.detach() {
            Some(this) => {
                parent.borrow_mut().children.push(this);
                self.index = parent.borrow().children.len() - 1;
                self.parent = Some(Rc::downgrade(&parent));
            }

            None => ()
        }
    }

    pub fn children(&self) -> &Vec<Rc<Cell<T>>> { &self.children }

    pub fn detach(&self) -> Option<Rc<Cell<T>>>
    {
        match self.parent() {
            Some(parent) => {
                let this =
                    parent
                        .borrow_mut()
                        .children
                        .swap_remove(self.index);

                parent
                    .borrow_mut()
                    .children[self.index]
                    .borrow_mut()
                    .index = self.index;

                Some(this)
            }

            None => None
        }
    }

    pub fn grandparent(&self) -> Option<Rc<Cell<T>>>
    {
        match self.parent() {
            Some(parent) => parent.borrow().parent(),
            None => None
        }
    }

    pub fn is_leaf(&self) -> bool { self.children.is_empty() }
    pub fn is_root(&self) -> bool { self.parent.is_none() }

    pub fn parent(&self) -> Option<Rc<Cell<T>>>
    {
        match self.parent {
            Some(ref parent) => parent.upgrade(),
            None => None
        }
    }

    pub fn set_value(&mut self, value: T) { self.value = value; }

    pub fn upgrade(&mut self)
    {
        match self.grandparent() {
            Some(grandparent) => self.attach(grandparent),
            None => ()
        }
    }

    pub fn value(&self) -> &T { &self.value }
}
