struct MyStack {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
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
    fn top(&self) -> i32 {
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
    pub fn test225() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        let ret_3: i32 = obj.top();
        assert_eq!(2, ret_3);
        let ret_2: i32 = obj.pop();
        assert_eq!(2, ret_2);
        let ret_4: bool = obj.empty();
        assert_eq!(false, ret_4);
        obj.pop();
        assert_eq!(true, obj.empty());
    }
}
