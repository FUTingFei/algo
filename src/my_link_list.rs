pub mod my_link_list {
    type NextNode<T> = Option<Box<ListNode<T>>>;

    #[derive(Clone, Debug)]
    pub struct ListNode<T> {
        val: T,
        next: NextNode<T>,
    }

    impl<T> ListNode<T> {
        fn new(val: T) -> Self {
            ListNode {
                val: val,
                next: None,
            }
        }

        fn set_next(&mut self, node: Option<Self>) {
            self.next = None;
            if let Some(x) = node {
                self.next = Some(Box::new(x));
            }
        }

        fn get_next<'a>(&'a mut self) -> Option<&'a mut Self> {
            if let Some(ref mut x) = self.next {
                return Some(x);
            }
            None
        }     

        fn get<'a>(&'a mut self, index: usize) -> Option<&'a mut Self> {
            if index == 0 {
                return Some(self);
            }

            if let Some(x) = self.get_next() {
                x.get(index - 1)
            } else {
                None
            }
        }

        fn get_value(&self) -> &T {
            &self.val
        }    

        fn get_last<'a>(&'a mut self) -> &'a mut Self {
            if let Some(ref mut x) = self.next {
                return x.get_last();
            }
            self
        }

        fn get_last_immutable<'a>(&'a self) -> &'a Self {
            if let Some(ref x) = self.next {
                return x.get_last_immutable();
            }
            self
        }

        fn push(&mut self, val: T) {
            let new_node = ListNode::new(val);
            self.get_last().set_next(Some(new_node));
        }
    }

    #[derive(Clone, Debug)]
    pub struct List<T> {
        len: usize,
        next: NextNode<T>,
    }
    
    impl<T> List<T> {
        pub fn new() -> Self {
            List { len: 0, next: None }
        }

        fn get_next<'a>(&'a mut self) -> Option<&'a mut ListNode<T>> {
            if let Some(ref mut x) = self.next {
                return x.get_next();
            }
            None
        }

        fn get<'a>(&'a mut self, index: usize) -> Option<&'a mut ListNode<T>> {
            if index > self.len || index == 0 {
                return None;
            }

            let node = self.get_next().unwrap();
            if index == 1 {
                return Some(node);
            }

            node.get(index - 1)
        }

        fn get_last<'a>(&'a mut self) -> Option<&'a mut ListNode<T>> {
            if let Some(ref mut x) = self.next {
                Some(x.get_last())
            } else {
                None
            }
        }

        fn get_last_immutable<'a>(&'a self) -> Option<&'a ListNode<T>> {
            if let Some(ref x) = self.next {
                Some(x.get_last_immutable())
            } else {
                None
            }
        }

        pub fn get_last_value(&self) -> Option<&T> {
            if self.len == 0 {
                return None;
            }
            Some(self.get_last_immutable().unwrap().get_value())
        }

        pub fn push(&mut self, elem: T) {
            if self.len == 0 {
                self.next = Some(Box::new(ListNode::new(elem)));
            } else {
                if let Some(ref mut x) = self.get_last() {
                    x.push(elem);
                }
            }
            self.len += 1;
        }

        pub fn pop(&mut self) {
            if self.len == 0 {
                return ();
            }
            self.len -= 1;
            let index = self.len;
            self.get(index - 1).unwrap().set_next(None);
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }
}