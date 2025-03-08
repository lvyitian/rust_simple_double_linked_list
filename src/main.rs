use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug,Clone)]
pub struct Node<T>{
    pub prev: Option<Weak<RefCell<Node<T>>>>,
    pub value:Rc<RefCell<T>>,
    pub next:Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Node<T> {
    pub fn new(prev: Option<Weak<RefCell<Node<T>>>>, value: T, next: Option<Rc<RefCell<Node<T>>>>)->Node<T>{
        Node{prev,value:Rc::new(RefCell::new(value)),next}
    }
}
#[derive(Debug)]
pub struct LinkedList<T>{
    pub head:Option<Rc<RefCell<Node<T>>>>,
}
impl<T> LinkedList<T>{
    pub fn new() -> LinkedList<T>{LinkedList{ head: None}}
    pub fn new_from_head_node(head: Rc<RefCell<Node<T>>>) -> LinkedList<T>{LinkedList{head: Some(head)}}
    pub fn create_from_existing_linked_list(list:LinkedList<T>)->LinkedList<T>{LinkedList{..list}}
    pub fn push_front(&mut self,value:T){
        let node = Some(Rc::new(RefCell::new(Node::new(None, value, self.head.clone()))));
        if let Some(head) = self.head.clone() {
            head.borrow_mut().prev=Some(Rc::downgrade(&node.clone().unwrap()));
        }
        self.head=node;
    }
    pub fn push_back(&mut self,value:T){
        let last_node=self.node_iter().last();
        let node = Some(Rc::new(RefCell::new(Node::new(last_node.clone().map(|i| Rc::downgrade(&i)), value, None))));
        if let Some(last_node) = last_node {
            last_node.borrow_mut().next=node.clone();
        }else{
            self.head=node;
        }
    }
    pub fn insert_after(&mut self,index: usize, value: T){
        let node=self.node_iter().nth(index);
        if let Some(node) = node {
            let new_node=Some(Rc::new(RefCell::new(Node::new(Some(Rc::downgrade(&node.clone())), value, node.borrow().next.clone()))));
            if let Some(t) = node.borrow().next.clone() {
                t.borrow_mut().prev=new_node.clone().map(|i|Rc::downgrade(&i));
            }
            node.borrow_mut().next=new_node;
        }else{
            self.push_back(value);
        }
    }
    pub fn insert_before(&mut self,index: usize, value: T){
        let node=self.node_iter().nth(index);
        if let Some(node) = node {
            if let Some(Some(prev)) = node.borrow().prev.clone().map(|i| i.upgrade()) {
                let new_node=Some(Rc::new(RefCell::new(Node::new(Some(Rc::downgrade(&prev.clone())), value, Some(node.clone())))));
                prev.borrow_mut().next=new_node.clone();
                node.borrow_mut().prev=new_node.map(|i| Rc::downgrade(&i));
            }else{
                self.push_front(value);
            }
        }else{
            self.push_back(value);
        }
    }
    pub fn remove_at(&mut self,index:usize)->Option<Rc<RefCell<T>>>{
        let node=self.node_iter().nth(index);
        if let Some(node) = node {
            if let Some(Some(prev))=node.borrow().prev.clone().map(|i| i.upgrade()) {
                prev.borrow_mut().next=node.borrow().next.clone();
                if let Some(next)=node.borrow().next.clone() {
                    next.borrow_mut().prev=Some(Rc::downgrade(&prev));
                }
            }else{
                self.head=node.borrow().next.clone();
            }
            Some(node.borrow().value.clone())
        }else{ None }
    }
    pub fn len(&self)->usize{
        let mut ret=0;
        for _ in self.iter(){
            ret+=1;
        }
        ret
    }
    pub fn iter(&self)->LinkedListIterator<T>{
        LinkedListIterator{current:self.head.clone()}
    }
    pub fn node_iter(&self)->LinkedListNodeIterator<T>{
        LinkedListNodeIterator{current:self.head.clone()}
    }
}
impl<T: Clone> Clone for LinkedList<T>{
    fn clone(&self)->LinkedList<T>{
        let mut ret=LinkedList::new();
        for i in self.iter() {
            ret.push_back(i.borrow().deref().clone());
        }
        ret
    }
}
#[derive(Debug,Clone)]
pub struct LinkedListIterator<T>{
    current:Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Iterator for LinkedListIterator<T> {
    type Item = Rc<RefCell<T>>;
    fn next(&mut self) -> Option<Self::Item>{
        let ret=self.current.clone();
        self.current=if let Some(node) = &self.current {node.borrow().next.clone()}else{None};
        ret.map(|i| i.borrow().value.clone())
    }
}
impl<T> LinkedListIterator<T>{
    pub fn peek(&self)->Option<Rc<RefCell<T>>>{self.current.clone().map(|i| i.borrow().value.clone())}
    pub fn peek_node(&self)->Option<Rc<RefCell<Node<T>>>>{self.current.clone()}
}
#[derive(Debug,Clone)]
pub struct LinkedListNodeIterator<T>{
    current:Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Iterator for LinkedListNodeIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item>{
        let ret=self.current.clone();
        self.current=if let Some(node) = &self.current {node.borrow().next.clone()}else{None};
        ret
    }
}
impl<T> LinkedListNodeIterator<T>{
    pub fn peek(&self)->Option<Rc<RefCell<Node<T>>>>{self.current.clone()}
}
fn main() {
    let mut list=LinkedList::<i32>::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("before remove:");
    for i in list.iter() {
        println!("{}",i.borrow());
    }
    list.remove_at(1);
    println!("after remove:");
    for i in list.iter() {
        println!("{}",i.borrow());
    }
}
