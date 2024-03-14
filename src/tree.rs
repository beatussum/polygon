use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

pub struct Cell<T>
{
    children: Vec<Rc<Node<T>>>,
    index: usize,
    parent: Option<Weak<Node<T>>>,
    value: T
}

pub struct Node<T>(RefCell<Cell<T>>);

impl<T> Cell<T> {
    pub fn children(&self) -> &Vec<Rc<Node<T>>> { &self.children }

    pub fn is_leaf(&self) -> bool { self.children.is_empty() }
    pub fn is_root(&self) -> bool { self.parent.is_none() }

    pub fn new(value: T) -> Self
    {
        Cell {
            children: Vec::new(),
            index: usize::default(),
            parent: None,
            value
        }
    }

    pub fn parent(&self) -> &Option<Weak<Node<T>>> { &self.parent }
    pub fn set_value(&mut self, value: T) { self.value = value; }
    pub fn value(&self) -> &T { &self.value }
}

impl<T> Node<T> {
    pub fn above(&self, n: usize)
        -> Result<Rc<Self>, (usize, Option<Rc<Self>>)>
    {
        match self.parent() {
            Some(mut parent) => {
                let mut i = n;
                let mut grandparent = parent.parent();

                while grandparent.is_some() {
                    if n == 0 {
                        break;
                    } else {
                        parent = grandparent.clone().unwrap();
                        grandparent = grandparent.unwrap().parent();
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

    pub fn attach(self: Rc<Self>, parent: Rc<Self>)
    {
        self.detach();

        self.borrow_mut().index = parent.borrow().children.len();
        self.borrow_mut().parent = Some(Rc::downgrade(&parent));
        parent.borrow_mut().children.push(self);
    }

    pub fn borrow(&self) -> Ref<Cell<T>> { self.0.borrow() }
    pub fn borrow_mut(&self) -> RefMut<Cell<T>> { self.0.borrow_mut() }

    pub fn detach(&self)
    {
        match self.parent() {
            Some(parent) => {
                parent
                    .borrow_mut()
                    .children
                    .swap_remove(self.borrow().index);

                parent
                    .borrow_mut()
                    .children[self.borrow().index]
                    .borrow_mut()
                    .index = self.borrow().index;
            }

            None => ()
        }
    }

    pub fn grandparent(&self) -> Option<Rc<Self>>
    {
        match self.parent() {
            Some(parent) => parent.parent(),
            None => None
        }
    }

    pub fn is_leaf(&self) -> bool { self.borrow().is_leaf() }
    pub fn is_root(&self) -> bool { self.borrow().is_root() }

    pub fn new(value: T) -> Rc<Self>
    {
        Rc::new(Node(RefCell::new(Cell::new(value))))
    }

    pub fn parent(&self) -> Option<Rc<Self>>
    {
        match self.borrow().parent() {
            Some(ref parent) => parent.upgrade(),
            None => None
        }
    }

    pub fn set_value(&self, value: T) { self.borrow_mut().set_value(value); }

    pub fn upgrade(self: Rc<Self>)
    {
        match self.grandparent() {
            Some(grandparent) => self.attach(grandparent),
            None => ()
        }
    }
}
