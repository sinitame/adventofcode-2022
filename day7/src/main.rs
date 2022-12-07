use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Directory<'a> {
    pub name: &'a str,
    pub size: Option<usize>,
    pub parent: Option<Rc<RefCell<Directory<'a>>>>,
    pub childs: Option<Vec<Rc<RefCell<Directory<'a>>>>>,
}

impl<'a> Debug for Directory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Directory")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("childs", &self.childs)
            .finish()
    }
}

impl<'a> Directory<'a> {
    pub fn cd_down(&self, dir_name: &str) -> Option<Rc<RefCell<Directory<'a>>>> {
        self.childs
            .as_ref()
            .map(|it| {
                it.iter()
                    .find(|it| it.borrow().name == dir_name)
                    .map(|it| it.clone())
            })
            .flatten()
    }

    pub fn cd_up(&self) -> Option<Rc<RefCell<Directory<'a>>>> {
        self.parent.as_ref().map(|it| it.clone())
    }

    pub fn compute_size(&mut self) -> usize {
        self.size
            .or_else(|| {
                let size = self
                    .childs
                    .as_ref()
                    .map(|it| it.iter().map(|it| it.borrow_mut().compute_size()).sum());
                self.size = size;
                size
            })
            .unwrap()
    }

    pub fn get_size(&self, max_size: usize, sizes: &mut Vec<usize>) {
        if let Some(size) = self.size {
            if self.childs.is_some() & (size <= max_size) {
                sizes.push(size);
            }
        }

        self.childs.as_ref().map(|it| {
            it.iter()
                .for_each(|child| child.borrow().get_size(max_size, sizes))
        });
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let root_dir = Rc::new(RefCell::new(Directory {
        name: "/",
        parent: None,
        size: None,
        childs: Some(Vec::new()),
    }));

    // Build file system hierarchy
    input.lines().skip(2).fold(root_dir.clone(), |cur_dir, it| {
        let line = it.split(" ").collect::<Vec<_>>();
        if line[0] != "$" {
            if line[0] == "dir" {
                let new_dir = Directory {
                    name: line[1],
                    parent: Some(cur_dir.clone()),
                    size: None,
                    childs: Some(Vec::new()),
                };
                cur_dir
                    .borrow_mut()
                    .childs
                    .as_mut()
                    .map(|it| it.push(Rc::new(RefCell::new(new_dir))));
            } else {
                let new_dir = Directory {
                    name: line[1],
                    parent: Some(cur_dir.clone()),
                    size: Some(line[0].parse::<usize>().unwrap()),
                    childs: None,
                };
                cur_dir
                    .borrow_mut()
                    .childs
                    .as_mut()
                    .map(|it| it.push(Rc::new(RefCell::new(new_dir))));
            }
        } else {
            if line[1] == "cd" {
                if line[2] != ".." {
                    return cur_dir.borrow().cd_down(line[2]).unwrap();
                } else {
                    return cur_dir.borrow().cd_up().unwrap();
                }
            }
        }
        cur_dir
    });

    // Complete the size of each directory
    let tot_size = root_dir.borrow_mut().compute_size();

    // Get directory sizes bellow 100000
    let mut sizes_bellow_100000 = Vec::new();
    root_dir.borrow().get_size(100000, &mut sizes_bellow_100000);

    // Problem 1 answer
    println!("{}", sizes_bellow_100000.iter().sum::<usize>());

    // Problem 2 answer
    let free_space = 70000000 - tot_size;
    let required_suppression = 30000000 - free_space;
    let mut sizes_bellow_30000000 = Vec::new();
    root_dir
        .borrow()
        .get_size(30000000, &mut sizes_bellow_30000000);
    let smaller_dir_to_delete = sizes_bellow_30000000
        .iter()
        .filter(|it| **it > required_suppression)
        .min()
        .unwrap();
    println!("{}", smaller_dir_to_delete);
}
