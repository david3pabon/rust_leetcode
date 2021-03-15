pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut current = &mut head;
    let mut p = l1;
    let mut q = l2;

    let mut carry = 0;

    while p != None || q != None {
        let sum = match (&p, &q) {
            (Some(l1), Some(l2)) => l1.val + l2.val + carry,
            (Some(l1), None) => l1.val + carry,
            (None, Some(l2)) => l2.val + carry,
            (None, None) =>  carry,
        };

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        q = if q != None { q.unwrap().next } else { q };
        p = if p != None { p.unwrap().next } else { p };
    }

    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    head.next
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> ListNode {
        ListNode {
            next: None,
            val
        }
    }
}

#[test]
fn test_add_two_numbers() {
    // Arrange
    let l1 = Some(Box::new(ListNode {
        next: Some(Box::new(ListNode {
            next: Some(Box::new(ListNode::new(3))),
            val: 4,
        })),
        val: 2,
    }));

    let l2 = Some(Box::new(ListNode {
        next: Some(Box::new(ListNode {
            next: Some(Box::new(ListNode::new(4))),
            val: 6,
        })),
        val: 5,
    }));

    // Act
    let result = add_two_numbers(l1, l2).unwrap();

    // Assert
    assert_eq!(7, (*result).val);

    let b = (*result).next.unwrap();
    assert_eq!(0, (*b).val);

    let c = (*b).next.unwrap();
    assert_eq!(8, (*c).val);
}