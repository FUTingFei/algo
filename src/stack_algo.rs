pub mod stack_algo {
    use std::fmt;

    pub struct ArrayStack {
        items: Vec<String>,
        count: usize,
        n: usize,
    }

    impl fmt::Debug for ArrayStack {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "items: {:?}, count: {}, size: {}", self.items, self.count, self.n)
        }
    }

    impl ArrayStack {
        pub fn new(n: usize) -> Self {
            ArrayStack {
                items: vec![],
                count: 0,
                n: n
            }
        }

        pub fn push(&mut self, item: String) -> bool {

            if self.count == self.n {
                return false;
            }
            
            self.items.push(item);
            self.count += 1;

            true
        }

        pub fn pop(&mut self) -> Option<String> {
            if self.count == 0 {
                return None;
            }

            let s = self.items.pop();
            self.count -= 1;
            s
        }
    }

    pub struct ArrayQueue {
        items: Vec<String>,
        n: usize,
        head: usize,
        tail: usize,
    }

    impl fmt::Debug for ArrayQueue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "items: {:?}, capacity: {}", self.items, self.n)
        }
    }

    impl ArrayQueue {
        pub fn new(size: usize) -> Self {
            ArrayQueue {
                items: vec![],
                n: size,
                head: 0,
                tail: 0,
            }
        }

        pub fn enqueue(&mut self, item: String) -> bool {
            if self.tail == self.n { 
                return false;
            }

            self.items.push(item);
            self.tail += 1;
            true

            // 循环队列
            // let t = (self.tail + 1) % self.n;
            // if t == self.head { 
            //     return false;
            // }

            // self.items.push(item);
            // self.tail = (self.tail + 1) % self.n;
            // true

        }

        pub fn dequeue(&mut self) -> Option<String> {
            if self.head == self.tail {
                return None;
            }
            self.head += 1;
            // 循环队列
            // self.head = (self.head + 1) % self.n;
            Some(self.items.remove(0))
        }

    }
}