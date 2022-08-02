

pub mod list; 

#[cfg(test)]
mod tests {
    use crate::list::*;
    use std::rc::Rc;

    #[test]
    fn append_insert_and_remove() {

        let mut linked_node : Rc<LinkedNode<u64>>= LinkedNode::new(10);
        linked_node = InmutList::append(linked_node, LinkedNode::new(15));
        let s = format!("{}", linked_node);
        assert_eq!(s, "[10,15]");
        linked_node = InmutList::insert_at(linked_node, LinkedNode::new(50), 0);
        let s = format!("{}", linked_node);
        assert_eq!(s, "[50,10,15]");
        linked_node = InmutList::insert_at(linked_node, LinkedNode::new(1), 2);
        let s = format!("{}", linked_node);
        assert_eq!(s, "[50,10,1,15]");
        linked_node = InmutList::insert_at(linked_node, LinkedNode::new(2), 0);
        let s = format!("{}", linked_node);
        assert_eq!(s, "[2,50,10,1,15]");
        linked_node = InmutList::insert_at(linked_node, LinkedNode::new(3), 3);
        let s = format!("{}", linked_node);
        assert_eq!(s, "[2,50,10,3,1,15]");
        
        linked_node = InmutList::remove_at(linked_node, 0, 1);
        let s = format!("{}", linked_node);
        assert_eq!(s, "[50,10,3,1,15]");

        linked_node = InmutList::remove_at(linked_node.clone(), InmutList::size(&linked_node) - 1, 1);
        let s = format!("{}", linked_node);
        assert_eq!(s, "[50,10,3,1]");

        linked_node = InmutList::remove_at(linked_node, 2, 1);
    
        let s = format!("{}", linked_node);
        assert_eq!(s, "[50,10,1]");
    }

    #[test]
    fn reverse() {
        let mut llist : Rc<LinkedNode<u64>> = LinkedNode::new(10);

        let mut s = format!("{}", llist);

        assert_eq!(s, "[10]");

        llist = InmutList::append(llist, LinkedNode::new(20));
        llist = llist.reverse();

        s = format!("{}", llist);
        assert_eq!(s, "[20,10]");

        llist = InmutList::insert_at(llist, LinkedNode::new(30),0);
        llist = llist.reverse();
        
        s = format!("{}", llist);
        assert_eq!(s, "[10,20,30]");

        for i in 4..11 {
            llist = InmutList::append(llist, LinkedNode::new(i * 10));
        }
        
        llist = llist.reverse();

        s = format!("{}", llist);
        assert_eq!(s, "[100,90,80,70,60,50,40,30,20,10]");
        
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
    fn dll_deque_operation() {
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
    fn dll_remove_node() {
        let mut dllist : DoublyLinkedList<u64> = DoublyLinkedList::new();
        let mut str :  String;

        for i in 0..7 {
            dllist.append(i);
        }

        let mut curr_node = dllist.at_as_ref(0).borrow().prev().as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().prev().as_ref().unwrap().clone();
        curr_node = curr_node.clone().borrow().prev().as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().prev().as_ref().unwrap().clone();
        curr_node = curr_node.clone().borrow().prev().as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().prev().as_ref().unwrap().clone();
        curr_node = curr_node.clone().borrow().prev().as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());

        str = format!("{}", dllist);
        assert_eq!(str, "[1,3,5]"); 
        
        curr_node = dllist.at_as_ref(0).borrow().prev().as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().next().as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());
        curr_node = curr_node.clone().borrow().next().as_ref().unwrap().clone();
        dllist.remove_node(curr_node.clone());

        str = format!("{}", dllist);
        assert_eq!(str, "[]");
    }

    #[test]
    fn dll_clone_test() {
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