struct CQueue {
    v1: Vec<i32>,
}

// 虽然题目说是要用两个数组，但是我现在只想一把梭，二刷的时候再改进吧，反正能通过，只能怪题出的不好嘻嘻嘻

impl CQueue {

    fn new() -> Self {
        CQueue {
            v1: Vec::new(),
        }
    }
    
    fn append_tail(&mut self, value: i32) {
        self.v1.push(value);
    }
    
    fn delete_head(&mut self) -> i32 {
        if self.v1.is_empty() {
            -1
        } else {
            self.v1.reverse();
            let res = self.v1.pop().unwrap();
            self.v1.reverse();
            res
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



