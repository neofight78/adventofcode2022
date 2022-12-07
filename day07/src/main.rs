use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

struct Directory<'a> {
    parent: Weak<RefCell<Self>>,
    directories: HashMap<&'a str, Rc<RefCell<Self>>>,
    files: Vec<usize>,
}

impl<'a> Directory<'a> {
    fn new(parent: Weak<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Directory {
            parent,
            directories: HashMap::new(),
            files: Vec::new(),
        }))
    }

    fn size(&self) -> usize {
        self.directories
            .iter()
            .map(|(_, d)| d.borrow().size())
            .sum::<usize>()
            + self.files.iter().sum::<usize>()
    }

    fn sub100k_size(&self) -> usize {
        self.directories
            .iter()
            .map(|(_, d)| d.borrow().sub100k_size())
            .sum::<usize>()
            + if self.size() <= 100000 {
                self.size()
            } else {
                0
            }
    }

    fn all_sizes(&self, sizes: &mut Vec<usize>) {
        self.directories
            .iter()
            .for_each(|(_, d)| d.borrow().all_sizes(sizes));
        sizes.push(self.size())
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let root = Directory::new(Weak::new());
    let mut current = root.clone();

    INPUT.split('\n').for_each(|l| {
        let mut tokens = l.split(' ');

        match tokens.next().unwrap() {
            "$" => {
                if tokens.next().unwrap() == "cd" {
                    match tokens.next().unwrap() {
                        "/" => current = root.clone(),
                        ".." => {
                            let parent = current.borrow().parent.upgrade().unwrap();
                            current = parent;
                        }
                        name => {
                            let directory = current.borrow().directories[name].clone();
                            current = directory;
                        }
                    }
                }
            }
            "dir" => {
                let name = tokens.next().unwrap();
                current
                    .borrow_mut()
                    .directories
                    .insert(name, Directory::new(Rc::downgrade(&current)));
            }
            size => current.borrow_mut().files.push(size.parse().unwrap()),
        }
    });

    const TOTAL_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;

    let sub100k = root.borrow().sub100k_size();

    let total_used = root.borrow().size();
    let minimum_to_delete = REQUIRED_SPACE - (TOTAL_SPACE - total_used);

    let mut sizes = Vec::new();
    root.borrow().all_sizes(&mut sizes);
    let smallest = sizes
        .iter()
        .filter(|&&s| s >= minimum_to_delete)
        .min()
        .unwrap();

    println!("Part 1: {sub100k}");
    println!("Part 2: {smallest}");
}
