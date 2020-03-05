struct MinStack {
    list: Vec<i32>,
    min: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            list: Vec::new(),
            min: i32::max_value(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.list.push(x);
        self.update_min();
    }   
    
    fn pop(&mut self) {
        self.list.pop();
        self.update_min();
    }
    
    fn top(&self) -> i32 {
        self.list[self.list.len() - 1]
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }

    fn update_min(&mut self) {
        if self.list.len() < 1 {
            self.min = i32::max_value();
            return
        }
        
        let mut min:i32 = self.list[0];

        for elem in &self.list {
            if *elem < min {
                min = *elem;
            }
        }

        self.min = min;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // --> Returns -3.
        min_stack.pop();
        assert_eq!(min_stack.top(), 0); // --> Returns 0.
        assert_eq!(min_stack.get_min(), -2); // --> Returns -2.[]
    }
}