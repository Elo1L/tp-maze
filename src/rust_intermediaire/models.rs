use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum Exploration {
    Explored,
    Unexplored
}

pub enum Maze {
    Branch {
        label: String,
        left: Rc<Maze>,
        right: Rc<Maze>,
        status: RefCell<Exploration>
    },
    Leaf {
        label: String
    }
}

impl Maze {
  pub fn explore(&self, work: &mut Vec<Rc<Maze>>, trace: &mut Vec<String>) {
    match self {
        Maze::Branch {
            label,
            left, 
            right, 
            status } => {
                let s = *status.borrow();
                match s {
                    Exploration::Explored => {},
                    Exploration::Unexplored =>   {
                        status.replace(Exploration::Explored);
                        work.push(Rc::clone(&right));
                        work.push(Rc::clone(&left));
                        trace.push(label.to_string());
                    }
                }
            }
        Maze::Leaf {label} => {trace.push(label.to_string())}
        }
    }
}


fn main() {
    let leaf2 = Rc::new(Maze::Leaf{label: String::from("2")});
    let leaf4 = Rc::new(Maze::Leaf{label:String::from("4")});
    let leaf5 = Rc::new(Maze::Leaf{label:String::from("5")});
    let leaf8 = Rc::new(Maze::Leaf{label:String::from("8")});
    let branch3 = Rc::new(Maze::Branch{label:String::from("3"), left:leaf4, right: leaf5.clone(), status: RefCell::new(Exploration::Unexplored)});
    let branch1 = Rc::new(Maze::Branch{label:String::from("1"), left:leaf2, right:branch3.clone(), status: RefCell::new(Exploration::Unexplored)});
    let branch7 = Rc::new(Maze::Branch{label:String::from("7"), left:leaf5.clone(), right:leaf8, status: RefCell::new(Exploration::Unexplored)});
    let branch6 = Rc::new(Maze::Branch{label:String::from("6"), left:branch3.clone(), right:branch7, status: RefCell::new(Exploration::Unexplored)});
    let branch0 = Maze::Branch{label:String::from("0"), left:branch1, right:branch6, status: RefCell::new(Exploration::Unexplored)};
    let mut work = vec![Rc::new(branch0)];
    let mut trace = vec![];
    while work.len() != 0 {
        let node = work.pop().expect("unexpected");
        node.explore(&mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }
}