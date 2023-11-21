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

    /// 查找节点
    pub fn find_node(&mut self, val: T) -> Option<&mut Box<Node<T>>> {
        let mut head = &mut self.head;
        while let Some(ref mut node) = head {
            if node.data == val {
                return Some(node);
            }
            head = &mut node.as_mut().next;
        }
        None
    }

    /// 删除节点
    pub fn remove(&mut self, val: T) -> bool {
        let mut head = &mut self.head;

        while let Some(ref mut node) = head {
            if node.data == val {
                self.head = node.next.take();
                return true;
            }

            if let Some(ref mut next) = node.next {
                if next.data == val {
                    node.next = next.next.take();
                    return true;
                }
            }

            head = &mut node.as_mut().next;
        }
        false
    }
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

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> 4 -> 5 -> None\n");
    }

    #[test]
    fn test_empty_linked_list() {
        let list = LinkedList::<i32>::new();
        assert_eq!(list.to_string(), "None\n");
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> 4 -> 5 -> None\n");

        list.reverse();
        assert_eq!(list.to_string(), "5 -> 4 -> 3 -> 2 -> 1 -> None\n");
    }

    #[test]
    fn test_remove() {
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> 4 -> 5 -> None\n");

        assert_eq!(list.remove(3), true);
        assert_eq!(list.to_string(), "1 -> 2 -> 4 -> 5 -> None\n");

        assert_eq!(list.remove(6), false);
        assert_eq!(list.to_string(), "1 -> 2 -> 4 -> 5 -> None\n");

        assert_eq!(list.remove(1), true);
        assert_eq!(list.to_string(), "2 -> 4 -> 5 -> None\n");

        assert_eq!(list.remove(5), true);
        assert_eq!(list.to_string(), "2 -> 4 -> None\n");

        assert_eq!(list.remove(2), true);
        assert_eq!(list.to_string(), "4 -> None\n");

        assert_eq!(list.remove(4), true);
        assert_eq!(list.to_string(), "None\n");
    }

    #[test]
    fn test_remove_existing() {
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        assert_eq!(list.remove(3), true);
        assert_eq!(list.to_string(), "1 -> 2 -> 4 -> 5 -> None\n");
    }

    #[test]
    fn test_remove_non_existing() {
        let mut list = LinkedList::new();
        list.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);

        assert_eq!(list.remove(6), false);
        assert_eq!(list.to_string(), "1 -> 2 -> 3 -> 4 -> 5 -> None\n");
    }
}
