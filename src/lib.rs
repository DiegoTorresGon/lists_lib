
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

pub mod list;

pub trait List<T> {
    /**
     * @brief inserts an element to the end of the list.
     * 
     * LinkedList implements this method in O(n) for time and O(1) for memory.
     * DoublyLinkedList implements this method in O(1) for both time and memory.
     * 
     * @param T value Element to be inserted.
     * 
     * @return u8 0 on success, 1 otherwise.
     */
    fn append(&mut self, value : T) -> u8;

    /**
     * #brief insert an element so it ends up in the specified index.
     * 
     * Both LinkedList and DoublyLinkedList implement this method in O(n) for time and O(1) for memory.
     * 
     * @param T value element to be inserted.
     * @param usize index The new element will be at this index.
     * 
     * @return u8 0 on success.
     */
    fn insert_at(&mut self, value : T, index : usize) -> u8;
        
    /**
     * @brief removes an element at the specified index from the list.
     * 
     * This method is implemented in O(n) for time and O(1) in DoublyLinkedList and LinkedList.
     * 
     * @param usize index The element at this index will be freed and replaced by the next element.
     * 
     * @return u8 0 on success, 1 if the list is already empy or 2 if index is out of bounds.
     */
    fn remove_at(&mut self, index : usize) -> u8;

    /**
     * @brief returns the value stored at the specified index.
     * 
     * This method is implemented in O(n) for time and O(1) for memory in the case of 
     *  DoublyLinkedList and LinkedList.
     * 
     * @param usize index the position of the desired element, starting from 0.
     * 
     * @return &T an inmutable borrow of the desired element stored inside the list.
     */
    fn value_at(&self, index : usize) -> &T;

    /**
     * @brief returns the value stored at the specified index.
     * 
     * This method is implemented in O(n) for time and O(1) for memory in the case of 
     *  DoublyLinkedList and LinkedList.
     * 
     * @param usize index the position of the desired element, starting from 0.
     * 
     * @return &T a mutable borrow of the desired element stored inside the list.
     */
    fn mut_value_at(&mut self, index : usize) -> &mut T;

    /**
     * @brief The number of elements stored in the list.
     * 
     * This methos is implements in O(1) for time and memory in DoublyLinkedList and LinkedList.
     * 
     * @return usize number of elements inside the list.
     */
    fn size(&self) -> usize;
}

pub trait Reversible {
    /**
     * @brief It reverses the original order of the list.
     * 
     * The list [0,1,2] will become [2,1,0].
     * 
     * @return &Self inmutable reference to self.
     */
    fn reverse(&mut self) -> &mut Self;
}

pub trait Deque<T>  {
    /**
     * @brief To check if the Deque is empty.
     * 
     * @return bool true if it's empty, false otherwise.
     */
    fn empty(&self) -> bool;

    /**
     * @brief Pushes an element to the end.
     * 
     * @return &mut Self A reference to itself.
     */
    fn push_back(&mut self, value : T) -> &mut Self;

    /**
     * @brief Pops the last element.
     * 
     * @return T Element that was previously at the back.
     */
    fn pop_back(&mut self) -> T;

    /**
     * @brief Pushes an element to the front of the list.
     * 
     * @return &mut Self Borrow of itself.
     */
    fn push_front(&mut self, value : T) -> &mut Self;

    /**
     * @brief Pops the element in the beggining.
     * 
     * @return T Element previously located at the beggining.
     */
    fn pop_front(&mut self) -> T;
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

#[derive(Debug)]
pub struct DoublyLinkedList<T : Clone + fmt::Display + std::convert::From<T>> {
    pub head : Option<Rc<RefCell<DoubleNode<T>>>>,
    pub n : usize,
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;
    use crate::List;
    use crate::Reversible;
    use crate::DoublyLinkedList;
    use crate::Deque;

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
    
    #[test]
    fn deque_operation() {
        let mut deq : DoublyLinkedList<u64> = DoublyLinkedList::new();

        let size : usize = 50;
        let mut arr : [u64; 50] = [0; 50];
        
        //Testing empty()
        assert_eq!(deq.empty(), true); 

        deq.push_back(1);
        assert_eq!(deq.empty(), false);

        //Testing push_back()
        deq.pop_back();

        for i in 0..(size) {
            deq.push_back(i as u64);
            arr[i as usize] = i as u64;
        }
        
        assert_eq!(deq.size(), size);
        for i in 0..(size) {
            assert_eq!(arr[i] , *deq.value_at(i));
        }

        //Testing pop_back()
        for i in (0..(size)).rev() {
            assert_eq!(deq.pop_back(), i as u64);
        }

        assert_eq!(deq.size(), 0 as usize);

        //Testing push_front()
        for i in 0..(size) {
            deq.push_front(i as u64);
            arr[size - 1 - i] = i as u64;
        }

        assert_eq!(deq.size(), size);
        for i in 0..(size) {
            assert_eq!(arr[i] , *deq.value_at(i));
        }

        //Testing pop_front()
        for i in (0..(size)).rev() {
            assert_eq!(deq.pop_front(), i as u64);
        }

        assert_eq!(deq.empty(), true);

    }

    #[test]
    fn remove_node() {
        let mut dllist : DoublyLinkedList<u64> = DoublyLinkedList::new();
        let mut str :  String;

        for i in 0..7 {
            dllist.append(i);
        }

        let mut curr_node = dllist.at_as_ref(0).borrow().prev.as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().prev.as_ref().unwrap().clone();
        curr_node = curr_node.clone().borrow().prev.as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().prev.as_ref().unwrap().clone();
        curr_node = curr_node.clone().borrow().prev.as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().prev.as_ref().unwrap().clone();
        curr_node = curr_node.clone().borrow().prev.as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());

        str = format!("{}", dllist);
        assert_eq!(str, "[1,3,5]"); 
        
        curr_node = dllist.at_as_ref(0).borrow().prev.as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().next.as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().next.as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());

        str = format!("{}", dllist);
        assert_eq!(str, "[]");
    }

    #[test]
    fn clone_test() {
        let mut dllist : DoublyLinkedList<u64> = DoublyLinkedList::new();
        let str :  String;
        let clone_str : String;

        for i in 0..7 {
            dllist.append(i);
        }

        let clone = dllist.clone();
        
        str = format!("{}", dllist);
        clone_str = format!("{}", clone);

        assert_eq!(str, clone_str);
        

    }

}