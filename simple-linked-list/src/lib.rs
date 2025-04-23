type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }

        len
    }

    pub fn push(&mut self, element: T) {
        if self.is_empty() {
            self.head = Some(Box::new(Node {
                data: element,
                next: None,
            }));
        } else {
            let new_node = Box::new(Node {
                data: element,
                next: self.head.take(),
            });

            self.head = Some(new_node);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();

        head.map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();

        if self.is_empty() {
            return reversed;
        }

        let mut current = self.head;

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = reversed.head.take();
            reversed.head = Some(node);
        }

        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for element in iter.into_iter() {
            list.push(element);
        }

        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();

        while let Some(element) = linked_list.pop() {
            vec.insert(0, element);
        }

        vec
    }
}
