struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    println!("Top element: {:?}", stack.peek());
    println!("Popped element: {:?}", stack.pop());
    println!("Popped element: {:?}", stack.pop());
}

