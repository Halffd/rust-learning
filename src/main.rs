struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, value: T) {
        self.data.push(value);
    }

    fn dequeue(&mut self) -> Option<T> {
        if !self.data.is_empty() {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);

    // Using raw pointers to access the front element
    let front_element: *const i32 = queue.peek().map_or(std::ptr::null(), |&x| &x);

    unsafe {
        if !front_element.is_null() {
            println!("Front element: {:?}", *front_element);
        } else {
            println!("Queue is empty, no front element.");
        }
    }

    println!("Dequeued element: {:?}", queue.dequeue());
    println!("Dequeued element: {:?}", queue.dequeue());
}
