//осознание до конца не пришло...

use std::cell::RefCell;
use std::rc::Rc;

type NodePtr = Rc<RefCell<Node>>;

struct Node {
    id:u32,
    adjacent:RefCell<Vec<NodePtr>>,
}

//Node → RefCell<Node> → Rc<RefCell<Node>> → NodePtr
impl Node{
    fn new(id:u32)->NodePtr{
        Rc::new(RefCell::new(Node 
            { id, adjacent: RefCell::new(Vec::new()) }
        ))
}

fn add_edge(&self, target:NodePtr){
    self.adjacent.borrow_mut().push(target);
}

}


fn main() {
   
let node1 = Node::new(1);
let node2 = Node::new(2);
let node3 = Node::new(3);

node1.borrow_mut().add_edge(Rc::clone(&node2));
node1.borrow_mut().add_edge(Rc::clone(&node3)); ////////
node2.borrow_mut().add_edge(Rc::clone(&node3));
node3.borrow_mut().add_edge(Rc::clone(&node1));

for elem in node1.borrow().adjacent.borrow().iter(){ //Notion конспект
    
println!(
    "Узел 1 указывает на Узел с ID: {}", 
    elem.borrow().id
    // node1.borrow().adjacent.borrow()[0].borrow().id
);
}
}