pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut lhs, mut rhs) = (l1, l2);
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    while let (Some(lnode), Some(rnode)) = (lhs.as_ref(), rhs.as_ref()) {
        if lnode.val <=  rnode.val {
            tail.next = lhs;
            tail = tail.next.as_mut().unwrap();
            lhs = tail.next.take();
        } else {
            tail.next = rhs;
            tail = tail.next.as_mut().unwrap();
            rhs = tail.next.take();
        }
    }

    tail.next = if lhs.is_some() {
        lhs
    } else {
        rhs
    };

    head.next

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = linkedlist![1,2,4];
        let l2 = linkedlist![1,3,4];
        let res = linkedlist![1,1,2,3,4,4];
        assert_eq!(res, merge_two_lists(l1,l2));
    }
}