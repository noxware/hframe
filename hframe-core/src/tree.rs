use std::{cell::RefCell, fmt::Debug, rc::Rc};

struct Handle<T>(Rc<RefCell<T>>);

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle(Rc::clone(&self.0))
    }
}

impl<T> Handle<T> {
    fn new(value: T) -> Self {
        Handle(Rc::new(RefCell::new(value)))
    }

    fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        match self.0.try_borrow() {
            Ok(value) => f(&value),
            Err(_) => panic!("The handle can't be read because it's being written somewhere else. Hint: Search where a `read_mut` closure is being used."),
        }
    }

    fn read_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self.0.try_borrow_mut() {
            Ok(mut value) => f(&mut value),
            Err(_) => panic!("The handle can't be written because it's being read somewhere else. Hint: Search where a `read` closure is being used."),
        }
    }
}

pub(crate) struct NodeData<T: Debug> {
    pub(crate) value: T,
    pub(crate) children: Vec<Node<T>>,
}

pub(crate) struct Node<T: Debug>(Handle<NodeData<T>>);

impl<T: Debug> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node(self.0.clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Walk {
    Continue,
    Stop,
}

impl<T: Debug> Node<T> {
    pub(crate) fn new(value: T) -> Self {
        Node(Handle::new(NodeData {
            value,
            children: Vec::new(),
        }))
    }

    pub(crate) fn nest(&self, node: Node<T>) -> Node<T> {
        self.read_mut(|data| data.children.push(node));
        self.clone()
    }

    fn walk_impl(&self, f: &mut impl FnMut(Node<T>, usize) -> Walk, depth: usize) -> Walk {
        if f(self.clone(), depth) == Walk::Stop {
            return Walk::Stop;
        }

        self.read(|data| {
            for child in data.children.iter() {
                if child.walk_impl(f, depth + 1) == Walk::Stop {
                    return Walk::Stop;
                }
            }

            Walk::Continue
        })
    }

    pub(crate) fn walk(&self, mut f: impl FnMut(Node<T>, usize) -> Walk) {
        self.walk_impl(&mut f, 0);
    }

    // kepp this private to fully hide the handle
    fn data(&self) -> Handle<NodeData<T>> {
        self.0.clone()
    }

    pub(crate) fn find(&self, predicate: impl Fn(Node<T>) -> bool) -> Option<Node<T>> {
        let mut found = None;

        self.walk(|node, _| {
            if predicate(node.clone()) {
                found = Some(node.clone());
                Walk::Stop
            } else {
                Walk::Continue
            }
        });

        found
    }

    pub(crate) fn read<R>(&self, f: impl FnOnce(&NodeData<T>) -> R) -> R {
        self.data().read(|data| f(&data))
    }

    pub(crate) fn read_mut<R>(&self, f: impl FnOnce(&mut NodeData<T>) -> R) -> R {
        self.data().read_mut(|data| f(data))
    }
}

impl Debug for Node<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.walk(|node, depth| {
                node.read(|data| {
                    writeln!(f, "{:indent$}{:?}", "", data.value, indent = depth * 2).unwrap();
                    Walk::Continue
                })
            });
        } else {
            self.read(|data| {
                write!(f, "{:?}", data.value).unwrap();

                if !data.children.is_empty() {
                    write!(f, " {{").unwrap();
                    for child in data.children.iter() {
                        write!(f, " {:?}", child).unwrap();
                    }
                    write!(f, " }}").unwrap();
                }
            });
        }

        Ok(())
    }
}
