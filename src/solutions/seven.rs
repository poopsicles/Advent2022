// Day 7: No Space Left On Device
// https://adventofcode.com/2022/day/7

// not fun :(
//
// Have a Folder class which holds Files and other Folders
// As you iterate through the input, keep the current folder
// on top of a stack, adding Files and modifying
// inner Folders using Rc<RefCell<T>>
//
// Recursively add every inner folder's size (when < 10k) and 
// get the smallest greater than the needed space

// Output:
// / (42586708)
// │  jztrccm.hvd (293559)
// ├─ bfqzjjct (30469934)
// │   │  lzrgqrgc (12679)
// │   │  phslrcw.ljl (240839)
// │   │  vntqgq.tps (169962)
// │   │  vzq.qvv (114950)
//
// (a few...hundred lines)
//
// │   │   │   │   │   │  jztrccm.hvd (269270)
// │   │   │   │   │   └─ lzrgqrgc.ddj (89807)
// │   │   │   │   ├─ phslrcw (785886)
// │   │   │   │   │   │  cgcqpjpn.zfz (124727)
// │   ├─ mwnmjjj (252881)
// │   │   │  mzwmp.jbw (184865)
// │   │   └─ thbh.nbg (68016)
// ├─ mqvn (287623)
// │   └─ spfzctll (287623)
//
// The sum of the folders of total size at most 100000 is 1453349.
// The smallest folder needed to be deleted has a size of 2948823.

struct ElfFile {
    name: String,
    size: u32,
}

impl ElfFile {
    fn new(name: &str, size: u32) -> Self {
        ElfFile {
            name: name.to_string(),
            size,
        }
    }
}

struct ElfFolder {
    name: String,
    files: Vec<ElfFile>,
    folders: Vec<Rc<RefCell<ElfFolder>>>,
}

impl ElfFolder {
    fn new(name: &str) -> Self {
        ElfFolder {
            name: name.to_string(),
            files: Vec::new(),
            folders: Vec::new(),
        }
    }

    fn get_size(&self) -> u32 {
        let mut size = self.files.iter().fold(0, |sum, x| sum + x.size);
        size += self
            .folders
            .iter()
            .fold(0, |sum, x| sum + x.borrow_mut().get_size());

        size
    }

    fn add_file(&mut self, file: ElfFile) {
        self.files.push(file);
    }

    fn add_folder(&mut self, folder: ElfFolder) {
        self.folders.push(Rc::new(RefCell::new(folder)));
    }

    fn find_folder(&self, name: &str) -> Rc<RefCell<ElfFolder>> {
        Rc::clone(
            self.folders
                .iter()
                .find(|x| x.borrow().name == name)
                .unwrap(),
        )
    }

    fn pretty(&self, indent: String) -> String {
        let mut res = String::new();

        res += &format!("{} ({})\n", self.name, self.get_size());

        if self.folders.is_empty() { // terminate the last file 
            let last = self.files.last().unwrap();

            for file in self.files.iter().rev().skip(1).rev() {
                res += &format!("{}│  {} ({})\n", indent, file.name, file.size);
            }

            res += &format!("{}└─ {} ({})\n", indent, last.name, last.size);
        } else {
            for file in self.files.iter() {
                res += &format!("{}│  {} ({})\n", indent, file.name, file.size);
            }
        }


        for folder in self.folders.iter() {
            res += &format!(
                "{}├─ {}",
                indent,
                folder.borrow().pretty(format!("{}│   ", indent))
            );
        }

        res
    }

    fn inner_sum_10k(&self) -> u32 {
        let mut sum = 0;

        for folder in self.folders.iter() {
            let size = folder.borrow().get_size();
            if size < 100001 {
                sum += size;
            }
            sum += folder.borrow().inner_sum_10k()
        }

        sum
    }

    fn smallest_needed(&self, needed: u32, target: u32) -> u32 {
        let mut res = target;

        for folder in self.folders.iter() {
            let size = folder.borrow().get_size();

            if size >= needed && size < res {
                res = size
            }

            res = folder.borrow().smallest_needed(needed, res);
        }

        res
    }
}

impl Display for ElfFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty(String::from("")))
    }
}

use std::{cell::RefCell, fmt::Display, fs::File, io::Read, rc::Rc};

pub fn solve(filename: &str) {
    let mut file = match File::open(filename) {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Error: does `{}` exist?", filename);
            return;
        }
    };
    
    let mut contents = String::new();
    let mut stack = Vec::new();

    file.read_to_string(&mut contents).unwrap();

    for chunk in contents.split('$').skip(1) {
        match &chunk[0..3] {
            " cd" => match chunk.split_once('\n').unwrap().0[4..].trim() {
                ".." => _ = stack.pop(),
                "/" => stack.push(Rc::new(RefCell::new(ElfFolder::new("/")))),
                x @ _ => {
                    let inner = stack.last().unwrap().borrow().find_folder(x);
                    stack.push(inner);
                }
            },

            " ls" => {
                for line in chunk.split_terminator('\n').skip(1).map(|x| x.trim()) {
                    if line.starts_with("dir") {
                        stack
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .add_folder(ElfFolder::new(&line[4..]));
                    } else {
                        let (size, name) = line.split_once(' ').unwrap();
                        stack
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .add_file(ElfFile::new(name, size.parse().unwrap()));
                    }
                }
            }

            _ => unreachable!(),
        }
    }

    let root = stack.first().unwrap().borrow();
    println!("{}", root);

    // Part 1
    println!("The sum of the folders of total size at most 100000 is {}.",  root.inner_sum_10k());

    // Part 2
    let needed = root.get_size() - 40000000;
    println!("The smallest folder needed to be deleted has a size of {}.", root.smallest_needed(needed, root.get_size()))
}
