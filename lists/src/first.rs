use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>)
}
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        return List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            next: mem::replace(&mut self.head, Link::Empty),
            elem: elem,
        });
        
        // println!("{:p}", new_node);

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                return None;
            }
            Link::More(node) => {
                let result = Some(node.elem);
                self.head = node.next;
                return result;
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        // Add more and allow to drop
        list.push(1);
        list.push(2);
        list.push(3);
    }
}