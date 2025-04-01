#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[allow(dead_code)]
pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut curr = head;
    let mut result = 0;
    while let Some(node) = curr {
        result = (result << 1) | node.val;
        curr = node.next;
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_linked_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;

        for &val in values {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }

        dummy.next
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(get_decimal_value(None), 0);
    }

    #[test]
    fn test_leetcode_examples() {
        assert_eq!(get_decimal_value(create_linked_list(&[1, 0, 1])), 5);
        assert_eq!(get_decimal_value(create_linked_list(&[0])), 0);
        assert_eq!(get_decimal_value(create_linked_list(&[1])), 1);
        assert_eq!(
            get_decimal_value(create_linked_list(&[
                1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0
            ])),
            18880
        );
    }
}
