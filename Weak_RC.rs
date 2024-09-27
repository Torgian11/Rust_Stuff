// Rc<T> has a strong_count and a weak_count
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node { // Node that owns its own children
    value: i32,
    parent: RefCell<Weak<Node>>, // This weak reference means the child node will have a reference to parent, but a weak one at that. So, it will never own the parent
    children: RefCell<Vec<Rc<Node>>>, // Nodes can be children of another node, so we wrap the Vec<Rc with a RefCell to allow mutability.
}

// The goal is to allow leaf to know about its parent Branch, and let Branch know about its parent Trunk.
// We will need to add a `parent` field to our Node struct. Trouble is, how do we define the type for `parent`?
// We cannot use Rc<T> because the `strong_count` values will never reach 0. This is because there will be a reference cycle,
// with leaf.parent pointing to branch, and branch.children pointing to leaf. So the counts will never reach 0, and we can get an unending cycle 
// which will crash the program.

// A parent node should own its children, and all children should be dropped if the parent is dropped. A child should never own its parent. 
//  This is a case for WEAK REFERENCES! 
//  Weak<T> is the answer!
fn main() {
    // Initially, Leaf has no reference to branch
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // Weak reference field ready to point to parent
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaef strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    // Initially, Branch has no reference to Trunk, and has reference to leaf
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Now, we need to set the leaf's `parent` field to weakly point to the branch
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        println!("leaf parent after change = {:?}", leaf.parent.borrow().upgrade());

        // Trunk has reference to branch
        let trunk = Rc::new(Node {
            value: 15,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&branch)]),
        });

        // DO the same mutation to the Branch's parent:
        *branch.parent.borrow_mut() = Rc::downgrade(&trunk);
        println!("Branche's parent: {:?}", branch.parent.borrow().upgrade());
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    
}
