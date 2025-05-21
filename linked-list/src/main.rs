type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn get(&self) -> &T {
        &self.value
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            value: item,
            next: None,
        });

        let mut current = &mut self.head;
        while let Some(ref mut node) = *current {
            current = &mut node.next;
        }
        *current = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    // TODO: optional

    // fn greatest(&self) -> Option<&T>
    // where
    //     T: Ord,
    // {
    //     let mut current = &self.head;
    //     let mut greatest: Option<&T> = None;
    //
    //     while let Some(ref node) = *current {
    //         let curr_val = node.get();
    //         if greatest.is_none() || curr_val > greatest.unwrap() {
    //             greatest = Some(curr_val);
    //         }
    //         current = &node.next;
    //     }
    //     greatest
    // }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(4);
    list.push(5);
    list.push(3);
    list.pop();

    if let Some(node) = &list.head {
        assert_eq!(node.get(), &2);
    }

    // assert_eq!(list.greatest(), Some(&5));
}