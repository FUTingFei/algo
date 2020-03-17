struct MyQueue {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            v: Vec::new(),
        }
    }
    
    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.v.push(x);
    }
    
    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.v.reverse();
        let res = self.v.pop().unwrap();
        self.v.reverse();
        res
    }
    
    /** Get the top element. */
    fn peek(&self) -> i32 {
        self.v[0]
    }
    
    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.v.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test232() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        let ret_3: i32 = obj.peek();
        assert_eq!(1, ret_3);
        let ret_2: i32 = obj.pop();
        assert_eq!(1, ret_2);
        let ret_4: bool = obj.empty();
        assert_eq!(false, ret_4);
        obj.pop();
        assert_eq!(true, obj.empty());
    }
}
