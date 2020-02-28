pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut fast = head.as_ref();
    let mut slow = head.as_ref();

    loop {

        if let Some(node) = fast {
            fast = node.next.as_ref();
        } else {
            break;
        }

        if let Some(node) = fast {
            fast = node.next.as_ref();
        } else {
            break;
        }

        if let Some(node) = slow {
            slow = node.next.as_ref();
        } else {
            break;
        }
    }

    let mid_addr = if let Some(node) = slow {
        node.as_ref() as *const ListNode
    } else {
        return None;
    };

    while let Some(node) = head.as_ref() {
        let addr = node.as_ref() as *const ListNode;
        if addr != mid_addr {
            head = head.unwrap().next;
        } else {
            break;
        }
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut head = linkedlist![1,2,3,4,5];
        let res = linkedlist![3,4,5];
        assert_eq!(res, middle_node(head));
    }

    #[test]
    fn test2() {
        let mut head = linkedlist![1,2,3,4,5,6];
        let res = linkedlist![4,5,6];
        assert_eq!(res, middle_node(head));
    }
}