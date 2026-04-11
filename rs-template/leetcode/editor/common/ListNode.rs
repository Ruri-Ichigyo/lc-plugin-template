#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        if values.is_empty() {
            return None;
        }
        
        let mut head = Some(Box::new(ListNode::new(values[0])));
        let mut current = head.as_mut().unwrap();
        
        for i in 1..values.len() {
            current.next = Some(Box::new(ListNode::new(values[i])));
            current = current.next.as_mut().unwrap();
        }
        
        head
    }

    pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;
        
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        
        result
    }

    pub fn print(head: Option<Box<ListNode>>) {
        let values = Self::to_vec(head);
        for (i, val) in values.iter().enumerate() {
            if i > 0 {
                print!("->");
            }
            print!("{}", val);
        }
        println!();
    }
}