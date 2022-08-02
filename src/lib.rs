

pub mod list {

    use std::fmt;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub trait List<T> {
        /**
        * @brief inserts an element to the end of the list.
        * 
        * @param T value Element to be inserted.
        * 
        * @return u8 0 on success, 1 otherwise.
        */
        fn append(&mut self, value : T) -> u8;

        /**
        * #brief insert an element so it ends up in the specified index.
        * 
        * DoublyLinkedList implements this method in O(n) for time and O(1) for memory.
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
        * This method is implemented in O(n) for time and O(1) in DoublyLinkedList.
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
        *  DoublyLinkedList.
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
        *  DoublyLinkedList.
        * 
        * @param usize index the position of the desired element, starting from 0.
        * 
        * @return &T a mutable borrow of the desired element stored inside the list.
        */
        fn mut_value_at(&mut self, index : usize) -> &mut T;

        /**
        * @brief The number of elements stored in the list.
        * 
        * This method is implements in O(1) for time and memory in DoublyLinkedList.
        * 
        * @return usize number of elements inside the list.
        */
        fn size(&self) -> usize;
    }

    pub trait InmutList<T> {
        /**
        * @brief inserts an element to the end of the list.
        * 
        * LinkedNode implements this method in O(n) for time and memory.
        * 
        * @param T value Element to be inserted.
        * 
        * @return Self A new list with the new value.
        */
        fn append(list : Self, other_list : Self) -> Self;

        /**
        * #brief insert an element so it ends up in the specified index.
        * 
        * LinkedNode implements this method in O(n) for time and memory.
        * 
        * @param T value element to be inserted.
        * @param usize index The new element will be at this index.
        * 
        * @return Self A new list with the inserted value.
        */
        fn insert_at(list : Self, insert_list : Self, index : usize) -> Self;
        
        /**
        * @brief removes an element at the specified index from the list.
        * 
        * This method is implemented in O(n) for time and memory in LinkedNode. 
        * 
        * @param usize index The element at this index will be removed and replaced by the next element.
        * 
        * @return Self a new list without the removed elements.
        */
        fn remove_at(list : Self, index : usize, count : usize) -> Self;

        /**
        * @brief returns the value stored at the specified index.
        * 
        * This method is implemented in O(n) for time and memory for LinkedNode. 
        * 
        * @param usize index the position of the desired element, starting from 0.
        * 
        * @return &T an inmutable borrow of the desired element stored inside the list.
        */
        fn value_at(list : Self, index : usize) -> T;

        /**
        * @brief The number of elements stored in the list.
        * 
        * This method is implemented in O(n) for time and memory in LinkedNode.
        * 
        * @return usize number of elements inside the list.
        */
        fn size(&self) -> usize;

        /**
        * @brief It reverses the original order of the list.
        * 
        * The list [0,1,2] will become [2,1,0].
        * 
        * This method is implemented in linear time for both time and memory.
        * 
        * @return Self A new list with the contents reversed.
        */
        fn reverse(&self) -> Self;
    }

    pub trait Reversible {
        /**
        * @brief It reverses the original order of the list.
        * 
        * The list [0,1,2] will become [2,1,0].
        * 
        * @return &mut Self mutable reference to self.
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
    pub struct LinkedNode<T : Clone + fmt::Display + std::convert::From<T>> {
        next : Option<Rc<LinkedNode<T>>>,
        value : T,
    }

    #[derive(Debug, Clone)]
    pub struct DoubleNode<T : Clone + fmt::Display + std::convert::From<T>> {
        next : Option<Rc<RefCell<DoubleNode<T>>>>,
        prev :  Option<Rc<RefCell<DoubleNode<T>>>>,
        value : T,
    }

    #[derive(Debug)]
    pub struct DoublyLinkedList<T : Clone + fmt::Display + std::convert::From<T>> {
        head : Option<Rc<RefCell<DoubleNode<T>>>>,
        n : usize,
    }


    //use crate::DoublyLinkedList;
    //use crate::List;
    //use crate::InmutList;
    //use crate::Deque;
    //use crate::LinkedNode;
    //use crate::DoubleNode;
    //use crate::Reversible;

    //use std::fmt;
    use std::string::String;
    use std::ops::AddAssign;
    use std::cmp;
    //use std::cell::RefCell;
    //use std::rc::Rc;

    //We will use a centinel value at each list.
    impl<T : Clone + fmt::Display + std::convert::From<T>> LinkedNode<T> {
        pub fn new(value : T) -> Rc<LinkedNode<T>> { 
            let new = LinkedNode {
                value : value.clone(), next : None, 
            };

            let rc_new = Rc::from(new);

            Rc::from(LinkedNode {
                value, next : Option::from(rc_new), 
            })
        }  

        //This creates an empty list. value parameter is needed because we don't know what type of value T will be.
        pub fn empty_list(value : T) -> Rc<LinkedNode<T>> {
            Rc::from(LinkedNode {
                value, next : None,
            })
        }

        pub fn has_next(&self) -> bool {
            if self.is_empty() {
                return false;
            }
            self.next.as_ref().unwrap().next.is_some()
        }

        pub fn next(&self) -> Rc<LinkedNode<T>> {
            if self.is_empty() {
                panic!("There is no next on empty list.");
            }
            let new_list = LinkedNode {
                value : self.value.clone(), next : self.next.as_ref().unwrap().next.clone() 
            };

            Rc::from(new_list)
        }

        pub fn cons(element : Rc<LinkedNode<T>>, list : Rc<LinkedNode<T>>) -> Rc<LinkedNode<T>> {
            if element.is_empty() {
                return list;
            }
            if list.is_empty() {
                return LinkedNode::new(element.value().clone());
            }

            let new  = LinkedNode {
                value : element.value().clone(),
                next : list.next.clone(),
            };

            let new_rc = Rc::from(new);

            Rc::from(LinkedNode {
                value : element.value.clone(),
                next : Option::from(new_rc), 
            })
        }

        pub fn is_empty(&self) -> bool {
            self.next.is_none()
        }

        pub fn value(&self) -> &T {
            if self.is_empty() {
                panic!("There is no value inside empty list.");
            }
            return &self.next.as_ref().unwrap().value; 
        }

        fn reverse_help(queued : Rc<LinkedNode<T>>, reversed : Rc<LinkedNode<T>>) -> Rc<LinkedNode<T>> {
            if queued.is_empty() {
                return reversed;
            }
            if !queued.has_next() {
                return InmutList::insert_at(reversed, LinkedNode::new(queued.value().clone()), 0);
            }
            else {
                return LinkedNode::reverse_help(queued.next(), 
                    InmutList::insert_at(reversed, LinkedNode::new(queued.value().clone()), 0))
            }
        }
    }

    impl<T>  fmt::Display for LinkedNode<T> 
        where T : fmt::Display + Clone + std::convert::From<T>
    { 
        fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
            let mut result = String::from("[");
            if !self.is_empty() {
                let mut list = Rc::from(self.clone()); 
            
                result += list.value().to_string().as_str();

                loop {
                    if list.has_next() {
                        list = list.next();

                        result.add_assign(",");
                        result.add_assign(list.value().to_string().as_str());
                    } else {
                        break;
                    } 
                }
            }
            result.add_assign("]");
            write!(f, "{}", result.as_str())
        }
    }

    impl<'a, T> InmutList<T> for Rc<LinkedNode<T>> 
        where T : Clone + fmt::Display + std::convert::From<T>
    {

        fn append(list : Rc<LinkedNode<T>>, other_list : Rc<LinkedNode<T>>) -> Rc<LinkedNode<T>> {
            if list.is_empty() {
                return other_list;
            }
            if !list.has_next() {
                return  LinkedNode::cons(list, other_list); 
            }
        
            LinkedNode::cons(list.clone(), Self::append(list.next(), other_list))
        } 

        //the inserted value will be at index.
        fn insert_at(list : Rc<LinkedNode<T>>, insert_list : Rc<LinkedNode<T>>, index : usize) -> Rc<LinkedNode<T>> {
            if index == 0 {
                return Self::append(insert_list, list);
            }                    
            if list.is_empty() {
                panic!("Invalid index at function insert_at.");
            }
        
            LinkedNode::cons(LinkedNode::new(list.value().clone()), 
                Self::insert_at(list.next(), insert_list, index - 1)) 
        }
    
        fn remove_at(list : Self, index : usize, count : usize) -> Self {
            if count == 0 {
                return list;
            }
            if index == 0 {
                if count == 1 && !list.has_next() {
                    return LinkedNode::empty_list(list.value.clone());
                } 
                if list.is_empty() {
                    panic!("Invalid count or index at remove_at function.");
                }
                return InmutList::remove_at(list.next(), index, count - 1);
            }
        
            LinkedNode::cons(list.clone(), Self::remove_at(list.next(), index - 1, count))
        }


        fn value_at(list : Self, index: usize) -> T {
            if index == 0 {
                return list.value().clone();
            }

            if list.has_next() {
                Self::value_at(list.next(), index - 1)
            } 
            else {
                panic!("Invalid index at value_at function.");
            }
        }

        fn size(&self) -> usize {
            if self.is_empty() {
                return 0;
            }
        
            let mut n : usize = 1;

            let mut list = self.clone();

            while list.has_next() {
                list = list.next();
                n += 1;
            }

            n
        }

        fn reverse(&self) -> Self {
            let size = self.size();

            if size < 2 {return self.clone();}

            LinkedNode::reverse_help(self.clone(), LinkedNode::empty_list(self.value.clone()))
        }

    }

    impl<T : Clone + fmt::Display + std::convert::From<T>> DoubleNode<T> {

        fn new(value : T) -> DoubleNode<T> { 
            DoubleNode {
                value, next : None, prev : None,
            }
        }
    }

    impl<T : Clone + fmt::Display + std::convert::From<T>> DoublyLinkedList<T> {
        pub fn new() -> DoublyLinkedList<T> {
            DoublyLinkedList {
                head : None, n : 0,
            }
        }

        pub fn at(&mut self, index : usize) -> Rc<RefCell<DoubleNode<T>>> {
            if index >= self.n {panic!("Invalid index")};
            
            let mut current = &mut self.head;
            if index >= self.n / 2 {
                for _ in (index)..(self.n) {
                    current = unsafe {&mut (*current.as_mut().unwrap().as_ptr()).prev}; 
                }
            } else {
                for _ in 0..(index){
                    current = unsafe {&mut (*current.as_mut().unwrap().as_ptr()).next};
                }
            }

            current.as_ref().unwrap().clone()
        }

        pub fn at_as_ref(&self, index : usize) -> &Rc<RefCell<DoubleNode<T>>> {
            if index >= self.n {panic!("Invalid index")};
            
            let mut current = &self.head;
            if index >= self.n / 2 {
                for _ in (index)..(self.n) {
                    current = unsafe {&mut (*current.as_ref().unwrap().as_ptr()).prev}; 
                }
            } else {
                for _ in 0..(index){
                    current = unsafe {&mut (*current.as_ref().unwrap().as_ptr()).next};
                }
            }

            current.as_ref().unwrap()
        }

        pub fn remove_node(&mut self, node : Rc<RefCell<DoubleNode<T>>>) -> DoubleNode<T> {
            let prev_node = match node.borrow().prev.clone() {
                Some(prev) => prev,
                None => panic!("Incomplete note at function remove_node.")
            };

            let next_node = match node.borrow().next.clone() {
                Some(next) => next,
                None => panic!("Incomplete note at function remove_node.")
            };
        
            prev_node.borrow_mut().next = Option::from(next_node.clone());
            next_node.borrow_mut().prev = Option::from(prev_node);

            if Rc::ptr_eq(self.head.as_ref().unwrap(), &node) {
                self.head = Option::from(next_node);
            }
        
            self.n -= 1;
            node.borrow().clone()
        }
    
    }

    impl<'a, T> List<T> for DoublyLinkedList<T> 
        where T : Clone + fmt::Display + std::convert::From<T>
    {

        fn append(&mut self, value : T) -> u8 {
            let mut new_node = DoubleNode::new(value); 
             
            if self.n == 0 {
                let node = Rc::from(RefCell::from(new_node)); 
                node.borrow_mut().prev = Option::from(node.clone());
                node.borrow_mut().next = Option::from(node.clone());
                self.head = Option::from(node);
                self.n += 1;
            } else {
                new_node.prev = Option::from(self.at(self.n - 1));
                new_node.next = Option::from(self.at(0));
                let cell_node = Rc::from(RefCell::from(new_node));
                self.at(&self.n-1).borrow_mut().next = Option::from(cell_node.clone());
                self.n += 1; 
        
                self.head.as_mut().unwrap().borrow_mut().prev = Option::from(cell_node);
            }
            0
        } 

        //the inserted value will be at index.
        fn insert_at(&mut self, value : T, index : usize) -> u8 {
            if index == self.n {
                self.append(value);
                return 0;
            } else if index >= self.n {
                panic!("Index out of bounds in function insert_at.");
            }

            let new_node = DoubleNode::new(value);

            let mut prev_node : Rc<RefCell<DoubleNode<T>>>;
            let mut next_node : Rc<RefCell<DoubleNode<T>>>;
            let inserting_node : Rc<RefCell<DoubleNode<T>>>;

            //Just to initialize values.
            prev_node = self.at(0);
            next_node = self.at(0);
        
            inserting_node = Rc::from(RefCell::from(new_node));

            let prev_index;
            let next_index;

            if index == 0 {prev_index = self.n - 1; next_index = 0;}
            else {prev_index = index - 1; next_index = index; }

            let max_index = cmp::max(prev_index, next_index);

            let mut current = &mut self.head;
            for i in 0..(max_index + 1) {
                if i == prev_index { prev_node = current.as_mut().unwrap().clone(); } 
                if i == next_index { next_node = current.as_mut().unwrap().clone(); } 

                if i != max_index { 
                    current = unsafe {&mut (*current.as_mut().unwrap().as_ptr()).next};
                }
            }

            inserting_node.borrow_mut().prev = Option::from(prev_node.clone());
            prev_node.borrow_mut().next = Option::from(inserting_node.clone());

            inserting_node.borrow_mut().next = Option::from(next_node.clone());
            next_node.borrow_mut().prev = Option::from(inserting_node.clone());
        
            if index == 0 {self.head = Option::from(inserting_node);}
            self.n += 1;

            0
        }
    
        fn remove_at(&mut self, index : usize) -> u8 {
            if self.n == 0 { return 1; }
            if index >= self.n {
                return 2;
            }

            if self.n == 1 {
                self.head = None;
                self.n = 0;
                return 0;
            }

            let to_be_deleted = self.at(index); 
        
            to_be_deleted.borrow().prev.as_ref().unwrap().borrow_mut().next = 
                Option::from(to_be_deleted.borrow().next.as_ref().unwrap().clone());

            to_be_deleted.borrow().next.as_ref().unwrap().borrow_mut().prev = 
                Option::from(to_be_deleted.borrow().prev.as_ref().unwrap().clone());

            if index == 0 { 
                self.head = Option::from(to_be_deleted.borrow().next.as_ref().unwrap().clone());
            }
            self.n -= 1;
            0
        }

        fn value_at(&self, index: usize) -> &T {
            return unsafe {&(*self.at_as_ref(index).as_ptr()).value}
        }
    
        fn mut_value_at(&mut self, index : usize) -> &mut T {
            return unsafe {&mut(*self.at_as_ref(index).as_ptr()).value}
        }

        fn size(&self) -> usize {
            self.n
        }
    }

    impl<T>  fmt::Display for DoublyLinkedList<T> 
        where T : fmt::Display + Clone + std::convert::From<T>
    { 
        fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
            let mut result = String::from("[");
            if self.size() != 0 {
                let line_append : &str;
                if self.value_at(0).to_string().len() > 10 {line_append = "\n";}
                else {line_append = "";}

                result += line_append;
                result.add_assign(self.value_at(0).to_string().as_str());

                for i in 1..(self.size()) {
                    result += ",";
                    result += line_append;
                    result.add_assign(self.value_at(i).to_string().as_str());
                };
            
                result += line_append;
            }
            result += "]";
            write!(f, "{}", result.as_str())
        }
    }

    impl<T> Reversible for DoublyLinkedList<T> 
        where T : Clone + fmt::Display + std::convert::From<T> 
    {

        fn reverse(&mut self) -> &mut Self {
            if self.n <= 1 {return self;}

            let mut current_node = self.head.as_ref().unwrap().clone();

            for i in 0..(self.n) {
                let prev_node = current_node.borrow().prev.as_ref().unwrap().clone();
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();

                current_node.borrow_mut().prev = Option::from(next_node.clone());
                current_node.borrow_mut().next = Option::from(prev_node);
            
                if i != self.n - 1 { current_node = next_node; }
            }
        
            self.head = Option::from(current_node);

            self
        }
    }

    impl<T> Clone for DoublyLinkedList<T>
        where T : Clone + fmt::Display + std::convert::From<T>
    {
        fn clone(&self) -> Self {
            if self.n == 0 {
                return DoublyLinkedList::new();
            }

            let mut new_list : DoublyLinkedList<T> = DoublyLinkedList::new();

            let mut curr_old = self.head.as_ref().unwrap().clone();

            for _ in 0..(self.n) {
                new_list.append(curr_old.borrow().value.clone());

            curr_old = curr_old.clone().borrow().next.as_ref().unwrap().clone();
            }

            new_list

        }
    }

    impl<T> Deque<T> for DoublyLinkedList<T>
        where T : Clone + fmt::Display + std::convert::From<T>
    {
        fn empty(&self) -> bool {
            self.size() == 0
        }

        fn push_back(&mut self, value : T) -> &mut Self {
            self.append(value);
            self
        }

        fn pop_back(&mut self) -> T {
            let removed = self.value_at(self.size() - 1).clone();
            self.remove_at(self.size() - 1);
            removed
        }

        fn push_front(&mut self, value : T) -> &mut Self {
            self.insert_at(value, 0);
            self
        }

        fn pop_front(&mut self) -> T {
            let removed = self.value_at(0).clone();
            self.remove_at(0);
            removed
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::List;
    use crate::list::Reversible;
    use crate::list::DoublyLinkedList;
    use crate::list::Deque;
    use crate::list::LinkedNode;
    use std::rc::Rc;
    use crate::list::InmutList;

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