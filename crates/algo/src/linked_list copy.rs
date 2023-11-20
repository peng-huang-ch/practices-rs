use std::fmt::{Debug, Display};

type Link<T> = Option<Box<Node<T>>>;

/// 单链表节点
#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    /// 在链表头部插入节点(头插法push front)
    pub fn prepend(&mut self, data: T) -> &mut Self {
        let mut node = Box::new(Node::new(data));
        let head = self.head.take();
        match head {
            Some(head) => {
                node.next = Some(head);
                self.head = Some(node);
            }
            None => self.head = Some(node),
        }
        self
    }

    /// 翻转链表
    pub fn reverse(&mut self) -> &mut Self {
        let mut prev = None;
        let mut next = self.head.take();

        while let Some(mut node) = next {
            next = node.next;
            node.next = prev;
            prev = Some(node);
        }

        self.head = prev;
        self
    }

    /// 删除节点
    pub fn remove(&mut self, val: T) -> bool {
        let mut next = &mut self.head;
        while let Some(mut node) = next.take() {
            if node.data == val {
                next.as_mut().unwrap().next = node.next;
            } else {
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            }
        }

        head_v.next
    }

    // /// 删除节点
    // pub fn remove(&mut self, val: T) -> bool {
    //     let mut prev: Option<Box<Node<T>>> = None;
    //     let mut next = self.head.take();

    //     while let Some(mut node) = next {
    //         if node.data == val {
    //             if prev.is_none() {
    //                 self.head = node.next.take();
    //                 return true
    //             }

    //             // if let Some(mut prev_node) = prev {
    //             //     prev_node.next = node.next;
    //             //     return true
    //             // }
    //             let next = node.next.take();
    //             prev.as_mut().unwrap().next = next;
    //             return true
    //         }
    //         next = node.next.take();
    //         prev = Some(node);
    //     }
    //     false
    // }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut next = self.head.as_ref();
        let mut res = String::new();
        while let Some(node) = next {
            res.push_str(&format!("{} -> ", node.data));
            next = node.next.as_ref();
        }
        write!(f, "{}None\n", res)
    }
}

mod test {

    #[test]
    fn test_linked_list() {
        use super::LinkedList;
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        println!("{list}");
    }

    #[test]
    fn test_empty_linked_list() {
        use super::LinkedList;
        let list = LinkedList::<i32>::new();
        println!("{list}");
    }

    #[test]
    fn test_reverse() {
        use super::LinkedList;
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> 4 -> 5 -> None\n");

        list.reverse();
        assert_eq!(list.to_string(), "5 -> 4 -> 3 -> 2 -> 1 -> None\n");
    }

    #[test]
    fn test_remove() {
        use super::LinkedList;
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> 4 -> 5 -> None\n");

        let removed = list.remove(1);
        assert_eq!(removed, true, "item has been removed.");
        assert_eq!(list.to_string(), "2 -> 3 -> 4 -> 5 -> None\n");

        let removed = list.remove(3);
        assert_eq!(removed, true, "item has been removed.");
        assert_eq!(list.to_string(), "2 -> 4 -> 5 -> None\n");
    }
}


  // /// 删除节点
    // pub fn remove(&mut self, val: T) -> bool {
    //     let mut node = &mut self.head;
    //     while let Some(prev) = node {
    //         while let Some(next) = &mut prev.next {
    //             if next.data == val {
    //                 prev.next = next.next.take();
    //                 return true;
    //             } else {
    //                 node = &mut prev.next;
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }