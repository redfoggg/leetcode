#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1.unwrap();
        let mut l2 = l2.unwrap();
        let mut carry = 0;
        let mut sum: i32;
        
        // [0] + [0]
        if (l1.val == 0 && l1.next.is_none()) && (l2.val == 0 && l2.next.is_none()){
            return Some(Box::new(ListNode::new(0)))
        }
        else if l2.val == 0 && l2.next.is_none() {
            return Some(l1)
        }
        else if l1.val == 0 && l1.next.is_none() {
            return Some(l2)
        }
        

        //first value  
        sum = l1.val + l2.val;
        if sum > 9 {
            carry = 1;
            sum -= 10;
        }
        
        //when there only one value to be added like [1] + [3]
        if l2.next.is_none() && l1.next.is_none() {
            let mut head = Box::new(ListNode::new(sum));
            if carry == 1 {
                head.next = Some(Box::new(ListNode::new(carry)));
            }
            return Some(head);
        }
        
        if l1.next.is_some() { l1 = l1.next.unwrap(); } else { l1.val = 0; }
        if l2.next.is_some() { l2 = l2.next.unwrap(); } else { l2.val = 0; }
        
        let mut head = Box::new(ListNode::new(sum));
        let mut current = &mut head;

        while l1.next.is_some() || l2.next.is_some() {
            
            println!("entrou no loop principal com valor l1: {} e l2: {}", l1.val, l2.val);
            sum = l1.val + l2.val + carry;
            carry = 0;
            println!("valor soma: {}", sum);
            if sum > 9 {
                carry = 1;
                sum -= 10;
            }
            current.next = Some(Box::new(ListNode::new(sum)));
            current = current.next.as_mut().unwrap();
            if l1.next.is_some() { l1 = l1.next.unwrap(); } else { l1.val = 0; }
            if l2.next.is_some() { l2 = l2.next.unwrap(); } else { l2.val = 0; }
            
        }
        //last value
        println!("fora do loop principal com valor l1: {} e l2: {}", l1.val, l2.val);
        sum = l1.val + l2.val + carry;
        let mut last_carry = 0;
        
        if sum > 9 {
            last_carry = 1;
            sum -= 10;
        }

        println!("valor soma: {}", sum);
        
        current.next = Some(Box::new(ListNode::new(sum)));
        current = current.next.as_mut().unwrap();
        if last_carry != 0 {
            current.next = Some(Box::new(ListNode::new(last_carry)));
        }

        Some(head)
    }
    
    pub fn print_linked_list(l: Option<Box<ListNode>>) {
        let mut l = l.unwrap();
        while l.next.is_some() {
            print!("{}", l.val);
            l = l.next.unwrap();
        }
        print!("{}", l.val);
    }
}

fn main(){
    let mut l1 = Box::new(ListNode::new(9));
    let mut l3 = ListNode::new(9);
    let l5 = ListNode::new(1);
    l3.next = Some(Box::new(l5.clone()));
    l1.next = Some(Box::new(l3.clone()));
    //[9, 9, 1] -> 199 

    let l2 = Box::new(ListNode::new(1));
    //[1] -> 1

    let result = Solution::add_two_numbers(Some(l1), Some(l2));
    Solution::print_linked_list(result);
    //result:[0, 0, 2] -> 200 
}
