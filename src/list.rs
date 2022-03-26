use crate::DoublyLinkedList;
use crate::List;
use crate::InmutList;
use crate::Deque;
use crate::LinkedNode;
use crate::DoubleNode;
use crate::Reversible;

use std::fmt;
use std::string::String;
use std::ops::AddAssign;
use std::cmp;
use std::cell::RefCell;
use std::rc::Rc;


impl<T : Clone + fmt::Display + std::convert::From<T>> LinkedNode<T> {
    pub fn new(value : T) -> Rc<LinkedNode<T>> { 
        let new = LinkedNode {
            value, next : None, 
        };

        Rc::from(new)
    }

    pub fn cons(element : Rc<LinkedNode<T>>, list : Rc<LinkedNode<T>>) -> Rc<LinkedNode<T>> {
        let new  = LinkedNode {
            value : element.value.clone(),
            next : Option::from(list),
        };

        Rc::from(new)
    }
}

impl<T>  fmt::Display for LinkedNode<T> 
    where T : fmt::Display + Clone + std::convert::From<T>
{ 
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("[");
        if self.next.is_some() {
            let mut list = Option::from(Rc::from(self.clone()));
            
            result += list.as_ref().unwrap().value.to_string().as_str();

            loop {
                if list.as_ref().unwrap().next.is_some() {
                    list = Option::from(list.as_ref().unwrap().next.as_ref().unwrap().clone());
                } 
                result.add_assign(",");
                result.add_assign(list.as_ref().unwrap().value.to_string().as_str());
                
                if list.as_ref().unwrap().next.is_none() {
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
        if list.next.is_none() {
            return  LinkedNode::cons(list, other_list); 
        }
        
        return LinkedNode::cons(list.clone(), Self::append(list.next.as_ref().unwrap().clone(), other_list));
    } 

    //the inserted value will be at index.
    fn insert_at(list : Rc<LinkedNode<T>>, insert_list : Rc<LinkedNode<T>>, index : usize) -> Rc<LinkedNode<T>> {
        if index == 0 {
            return Self::append(insert_list, list);
        }                    
        if index == 1 {  
            let next : Self;
            if list.next.is_some() {next = Self::append(insert_list, list.next.as_ref().unwrap().clone());}
            else {next = insert_list;}

            return Self::append(list, next);
        }
        
        LinkedNode::cons(LinkedNode::new(list.value.clone()), Self::insert_at(list.next.as_ref().unwrap().clone(), insert_list, index - 1)) 
    }
    
    fn remove_at(list : Self, index : usize, count : usize) -> Self {
        if count == 0 {
            return list;
        }
        if index == 0 {
            match list.next.as_ref() {
                Some(next) => return Self::remove_at(next.clone(), index, count - 1),
                None => panic!("Invalid index as remove_at function.")
            }
        }
        
        let next = match list.next.as_ref() {
            Some(next) => next.clone(),
            None => panic!("Invalid index at remove_at function.")
        };

        return Rc::from(LinkedNode::cons(list, Self::remove_at(next, index - 1, count)));
    }

    fn value_at(list : Self, index: usize) -> T {
        if index == 0 {
            return list.value.clone();
        }

        let next = match list.next.as_ref() {
            Some(next) => next.clone(),
            None => panic!("Invalid index."),
        };

        return Self::value_at(next, index - 1);
    }

    fn size(&self) -> usize {
        let mut n : usize = 1;

        let mut list = self.clone();

        while list.next.is_some() {
            list = list.next.as_ref().unwrap().clone();
            n += 1;
        }

        n
    }
}

//impl<T> Reversible for LinkedNode<T> 
    //where T : Clone + fmt::Display + std::convert::From<T> 
//{

    //fn reverse(list : Self) -> &mut Self {
        //if self.  {return self;}
        //if self.n == 2 {
            //let mut new_head = self.at(1); 
            //new_head.next = Option::from(self.at(0));
            //self.head = Option::from(new_head);

            //self.tail = &mut self.at_as_mut(0).next;
            //return self;
        //}

        ////List is at least of length 3.
        //let mut after = self.at(2);
        //let mut middle = self.at(1);
        //let mut before = self.at(0);
        
        //for _ in 1..(self.n - 2) {
            //middle.next = Option::from(before);
            //before = middle;
            //middle = after;
            //after = match middle.next {
                //Some(next) => next,
                //None => panic!("Incorrect at reverse funcction.")
            //} 
        //}

        //middle.next = Option::from(before);
        //after.next = Option::from(middle);
        //self.head = Option::from(after);
    
        //self.tail = &mut self.at_as_mut(self.n - 2).next;

        //self
    //}
//}

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