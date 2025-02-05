pub mod std {
    pub mod collections {
        pub struct Node<T> {
            pub value: T,
            pub next: Option<Box<Node<T>>>,
            pub prev: Option<*mut Node<T>>,
        }

        pub struct LinkedList<T> {
            head: Option<Box<Node<T>>>,
            tail: Option<*mut Node<T>>,
            len: usize,
        }

        impl<T> LinkedList<T> {
            pub fn new() -> Self {
                LinkedList {
                    head: None,
                    tail: None,
                    len: 0,
                }
            }

            pub fn push_front_node(&mut self, mut node: Box<Node<T>>) {
                unsafe {
                    node.prev = None;
                    node.next = self.head.take();

                    if let Some(head) = &mut node.next {
                        // Corrected line: Cast the mutable reference to a raw pointer
                        head.prev = Some((&mut **head) as *mut Node<T>);
                    }

                    self.head = Some(node);
                    self.len += 1;

                    if self.len == 1 {
                        self.tail = self.head.as_mut().map(|h| h.as_mut() as *mut _);
                    }
                }
            }

            pub fn len(&self) -> usize {
                self.len
            }
        }
    }
}

fn main() {
    let mut list = std::collections::LinkedList::new();
    let node1 = Box::new(std::collections::Node {
        value: 10,
        next: None,
        prev: None,
    });

    list.push_front_node(node1);

    println!("List length: {}", list.len());
}
