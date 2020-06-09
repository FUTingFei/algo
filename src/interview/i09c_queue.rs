struct CQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl CQueue {

    fn new() -> Self {
        CQueue {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }
    
    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }
    
    fn delete_head(&mut self) -> i32 {
        if self.stack2.is_empty() {
            while let Some(n) = self.stack1.pop() {
                self.stack2.push(n);
            }
        }
        
        if let Some(n) =  self.stack2.pop() {
            n
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i09() {
        let mut obj = CQueue::new();
        obj.append_tail(3);
        assert_eq!(3, obj.delete_head());
        assert_eq!(-1, obj.delete_head());

        let mut obj = CQueue::new();
        assert_eq!(-1, obj.delete_head());
        obj.append_tail(5);
        obj.append_tail(2);
        assert_eq!(5, obj.delete_head());
        assert_eq!(2, obj.delete_head());
        
    }   
}



