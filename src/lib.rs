
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

pub mod list;

pub trait List<T> {
    fn append(&mut self, value : T) -> u8;

    fn insert_at(&mut self, value : T, index : usize) -> u8;
        
    fn remove_at(&mut self, index : usize) -> u8;

    fn value_at(&self, index : usize) -> &T;

    fn size(&self) -> usize;
}

pub trait Reversible {
    fn reverse(&mut self) -> &mut Self;
}

#[derive(Debug, Clone)]
struct Node<T : Clone + fmt::Display + std::convert::From<T>> {
    pub next : Option<Box<Node<T>>>,
    pub value : T,
}

#[derive(Debug, Clone)]
pub struct DoubleNode<T : Clone + fmt::Display + std::convert::From<T>> {
    pub next : Option<Rc<RefCell<DoubleNode<T>>>>,
    pub prev :  Option<Rc<RefCell<DoubleNode<T>>>>,
    pub value : T,
}

#[derive(Debug, Clone)]
pub struct LinkedList<T : Clone + fmt::Display + std::convert::From<T>> {
    head : Option<Box<Node<T>>>, 
    tail : *mut Option<Box<Node<T>>>,
    n : usize,
}

#[derive(Debug, Clone)]
pub struct DoublyLinkedList<T : Clone + fmt::Display + std::convert::From<T>> {
    head : Option<Rc<RefCell<DoubleNode<T>>>>,
    n : usize,
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;
    use crate::List;
    use crate::Reversible;
    use crate::DoublyLinkedList;

    #[test]
    fn append_insert_and_remove() {

        let mut linked_list : LinkedList<u64> = LinkedList::new();
        linked_list.append(10);
        linked_list.append(15);
        linked_list.insert_at(50, 0);
        linked_list.insert_at(1,2);
        linked_list.insert_at(2,0);
        linked_list.insert_at(3,3);

        linked_list.remove_at(0);
        linked_list.remove_at(linked_list.n - 1);
        linked_list.remove_at(2);
    
        let s = format!("{}", linked_list);
        assert_eq!(s, "[50,10,1]");
    }

    #[test]
    fn reverse() {
        let mut llist : LinkedList<u64> = LinkedList::new();
        llist.append(10);

        let mut s = format!("{}", llist);

        assert_eq!(s, "[10]");

        llist.append(20);
        s = format!("{}", llist.reverse());

        assert_eq!(s, "[20,10]");

        llist.insert_at(30,0);
        s = format!("{}", llist.reverse());

        assert_eq!(s, "[10,20,30]");
    }

    #[test]
    fn dllist_append_at() {
        let mut dllist : DoublyLinkedList<u64> = DoublyLinkedList::new();
        let mut s :  String;

        dllist.append(15); //15
        dllist.append(10); //15,10
        s = format!("{}", dllist);
        assert_eq!(s, "[15,10]");
        dllist.insert_at(50, 0); //50,15,10
        s = format!("{}", dllist);
        assert_eq!(s, "[50,15,10]");
        dllist.insert_at(1,2); //50,15,1,10
        s = format!("{}", dllist);
        assert_eq!(s, "[50,15,1,10]");
        dllist.insert_at(2,0); //2,50,15,1,10
        s = format!("{}", dllist);
        assert_eq!(s, "[2,50,15,1,10]");
        dllist.insert_at(3,3); //2,50,15,3,1,10
        
        s = format!("{}", dllist);
        assert_eq!(s, "[2,50,15,3,1,10]");

        dllist.append(3);
        s = format!("{}", dllist);

        assert_eq!(s, "[2,50,15,3,1,10,3]");
    }

    #[test]
    fn dllist_remove_test() {
        let mut dllist : DoublyLinkedList<u64> = DoublyLinkedList::new();
        let mut str :  String;

        for i in 0..7 {
            dllist.append(i);
        }

        for s in (0..7).step_by(2).rev() {
            dllist.remove_at(s);
        } 

        str = format!("{}", dllist);
        assert_eq!(str, "[1,3,5]"); 
        
        for _ in 0..3 {
            dllist.remove_at(0);
        }

        str = format!("{}", dllist);
        assert_eq!(str, "[]");
    }

    #[test]
    fn dllist_reverse() {
        let mut dllist : DoublyLinkedList<u64> = DoublyLinkedList::new();
        dllist.append(10);

        let mut s = format!("{}", dllist);

        assert_eq!(s, "[10]");

        dllist.append(20);
        s = format!("{}", dllist.reverse());

        assert_eq!(s, "[20,10]");

        dllist.insert_at(30,0);
        s = format!("{}", dllist.reverse());

        assert_eq!(s, "[10,20,30]");
    }
}