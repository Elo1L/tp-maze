use std::cell::RefCell;

#[derive(Copy, Clone)]
pub enum Exploration {
    Explored,
    Unexplored,
}

pub enum Maze<'a> {
    Branch {
        label: String,
        left: &'a Maze<'a>,
        right: &'a Maze<'a>,
        status: RefCell<Exploration>,
    },
    Leaf {
        label: String,
    },
}

impl Maze<'_> {
    pub fn explore(&self) -> Vec<String> {
        match self {
            Maze::Branch {
                label,
                left,
                right,
                status,
            } => {
                let s = *status.borrow();
                match s {
                    Exploration::Explored => vec![label.clone()],
                    Exploration::Unexplored => {
                        status.replace(Exploration::Explored);
                        let trace = &mut vec![label.to_string()];
                        let l = &left.explore();
                        let r = &right.explore();
                        trace.extend(l.clone());
                        trace.extend(r.clone());
                        (&trace).to_vec()
                    }
                }
            }
            Maze::Leaf { label } => {
                vec![label.clone()]
            }
        }
    }
}

fn main() {
    let leaf2 = Maze::Leaf {
        label: String::from("2"),
    };
    let leaf4 = Maze::Leaf {
        label: String::from("4"),
    };
    let leaf5 = Maze::Leaf {
        label: String::from("5"),
    };
    let leaf8 = Maze::Leaf {
        label: String::from("8"),
    };
    let branch3 = Maze::Branch {
        label: String::from("3"),
        left: &leaf4,
        right: &leaf5,
        status: RefCell::new(Exploration::Unexplored),
    };
    let branch1 = Maze::Branch {
        label: String::from("1"),
        left: &leaf2,
        right: &branch3,
        status: RefCell::new(Exploration::Unexplored),
    };
    let branch7 = Maze::Branch {
        label: String::from("7"),
        left: &leaf5,
        right: &leaf8,
        status: RefCell::new(Exploration::Unexplored),
    };
    let branch6 = Maze::Branch {
        label: String::from("6"),
        left: &branch3,
        right: &branch7,
        status: RefCell::new(Exploration::Unexplored),
    };
    let branch0 = Maze::Branch {
        label: String::from("0"),
        left: &branch1,
        right: &branch6,
        status: RefCell::new(Exploration::Unexplored),
    };
    //println!(branch0);
    println!("{:?}", branch0.explore());
    //let trace = ListBuffer[String]();
}
