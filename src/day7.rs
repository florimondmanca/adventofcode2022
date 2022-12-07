use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run() {
    let content = include_str!("inputs/7.txt");

    let root = parse(content);

    let mut dirs = vec![Rc::clone(&root)];

    let mut total_size_under_100k = 0;

    let root_size = root.total_size();
    let min_freed_size = 30000000 - (70000000 - root_size);
    let mut smallest_freed_dir_size = root_size;

    while let Some(dir) = dirs.pop() {
        for d in dir.sub_dirs.borrow().values() {
            dirs.push(Rc::clone(d));
        }
        let size = dir.total_size();
        if size < 100_000 {
            total_size_under_100k += size;
        }
        if size >= min_freed_size && size < smallest_freed_dir_size {
            smallest_freed_dir_size = size;
        }
    }

    println!("Answer (part 1): {total_size_under_100k}");
    println!("Answer (part 2): {smallest_freed_dir_size}");
}

struct Directory {
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    sub_dirs: RefCell<HashMap<String, Rc<Directory>>>,
}

impl Directory {
    fn new(parent: Option<Rc<Directory>>) -> Self {
        Directory {
            size: RefCell::new(0),
            parent,
            sub_dirs: RefCell::new(HashMap::new()),
        }
    }

    fn total_size(&self) -> usize {
        return *self.size.borrow()
            + self
                .sub_dirs
                .borrow()
                .values()
                .map(|subdir| subdir.total_size())
                .sum::<usize>();
    }
}

fn parse(content: &str) -> Rc<Directory> {
    let root = Rc::new(Directory::new(None));

    let mut cwd = Rc::clone(&root);

    for line in content.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();

        match (parts[0], parts[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match parts[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                dirname => {
                    let d = cwd.sub_dirs.borrow().get(dirname).unwrap().clone();
                    cwd = d;
                }
            }
            ("dir", dirname) => {
                let parent = Some(Rc::clone(&cwd));
                let sub_dir = Rc::new(Directory::new(parent));
                cwd.sub_dirs
                    .borrow_mut()
                    .insert(dirname.to_string(), sub_dir);
            }
            (size, _) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    return root;
}
