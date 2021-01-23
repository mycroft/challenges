use std::iter::FromIterator;

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            let mut curr = &self.head;
            let mut z = 0;

            while let Some(next) = curr {
                curr = &next.next;
                z += 1;
            }

            z
        }
    }

    pub fn push(&mut self, _element: T) {
        let node = Node {
            next: self.head.take(),
            value: _element
        };

        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let node = self.head.take().unwrap();
        self.head = node.next;

        Some(node.value)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.head.as_ref().unwrap().value)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::<T>::new();
        let mut curr = self.head;

        while let Some(node) = curr {
            result.push(node.value);
            curr  = node.next;
        }

        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        _iter
            .into_iter()
            .fold(SimpleLinkedList::<T>::new(), |mut l, k| {
                l.push(k);
                l
            })
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut l = vec![];
        let mut curr = self.head;

        while let Some(node) = curr {
            l.insert(0, node.value);
            curr = node.next;
        }

        l
    }
}
