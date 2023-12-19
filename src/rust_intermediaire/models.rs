use std::cell::RefCell;
use std::rc::Rc;

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
                match *status.borrow() {
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
    let leaf2 = Maze::Leaf{label: String::from("2")};
    let leaf4 = Maze::Leaf{label:String::from("4")};
    let leaf5 = Maze::Leaf{label:String::from("5")};
    let leaf8 = Maze::Leaf{label:String::from("8")};
    let branch3 = Maze::Branch{label:String::from("3"), left:Rc::new(leaf4), right: Rc::new(leaf5), status: RefCell::new(Exploration::Unexplored)};
    let branch1 = Maze::Branch{label:String::from("1"), left:Rc::new(leaf2), right: Rc::new(branch3), status: RefCell::new(Exploration::Unexplored)};
    let branch7 = Maze::Branch{label:String::from("7"), left:Rc::new(leaf5), right: Rc::new(leaf8), status: RefCell::new(Exploration::Unexplored)};
    let branch6 = Maze::Branch{label:String::from("6"), left:Rc::new(branch3), right: Rc::new(branch7), status: RefCell::new(Exploration::Unexplored)};
    let branch0 = Maze::Branch{label:String::from("0"), left:Rc::new(branch1), right:Rc::new(branch6), status: RefCell::new(Exploration::Unexplored)};
    let mut work = vec![Rc::new(branch0)];
    let mut trace = vec![];
    while work.len() != 0 {
        let node = work.pop().expect("unexpected");
        node.explore(&mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }
    //println!(branch0);
    //println!(branch0.explore());
    //let trace = ListBuffer[String]();
}