use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
struct Dir {
    parent: Option<Box<Dir>>,
    name: String,
    files: Vec<File>,
    sub_dirs: Vec<Dir>,
}

impl Dir {
    fn new(name: &str) -> Self {
        return Dir {
            parent: None,
            name: name.to_string(),
            files: Vec::new(),
            sub_dirs: Vec::new(),
        };
    }

    fn add_dir(&mut self, parent_dir: &str, dir_name: &str) {
        if parent_dir == self.name {
            self.sub_dirs.push(Dir::new(dir_name));
        } else {
            self.sub_dirs
                .iter_mut()
                .for_each(|sub_dir| sub_dir.add_dir(parent_dir, dir_name));
        }
    }

    fn add_file(&mut self, parent_dir: &str, file_name: &str, size: usize) {
        if self.name == parent_dir {
            self.files.push(File::new(file_name, size));
        } else {
            self.sub_dirs
                .iter_mut()
                .for_each(|sub_dir| sub_dir.add_file(parent_dir, file_name, size));
        }
    }

    fn size(&self) -> usize {
        return self.files.iter().map(|file| file.size).sum::<usize>()
            + self
                .sub_dirs
                .iter()
                .map(|sub_dir| sub_dir.size())
                .sum::<usize>();
    }

    fn size_of_dirs(&self) -> Vec<usize> {
        let own_size = self.size();

        let dirs_size = self
            .sub_dirs
            .iter()
            .flat_map(|sub_dir| sub_dir.size_of_dirs());

        let mut all_sizes: Vec<usize> = dirs_size.collect();
        all_sizes.push(own_size);
        return all_sizes;
    }

    fn sum_size_of_dirs_lower_than(&self) -> usize {
        self.size_of_dirs()
            .iter()
            .filter(|size| **size < 100000)
            .sum()
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_dir: &str = "";
        Ok(s.lines().fold(Dir::new("/"), |mut dir, line| {
            return match line.split_once(" ").unwrap() {
                ("$", command) => {
                    if command.starts_with("cd") {
                        current_dir = command
                            .split_whitespace()
                            .last()
                            .expect("to not be an empty line");
                    }
                    dir
                }
                ("dir", dir_name) => {
                    dir.add_dir(current_dir, dir_name);
                    dir
                }
                (file_size, file_name) => {
                    dir.add_file(
                        current_dir,
                        file_name,
                        file_size
                            .parse::<usize>()
                            .expect("first argument to be usize"),
                    );
                    dir
                }
            };
        }))
    }
}

trait Node {
    fn size(&self) -> usize;
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        return File {
            name: name.to_string(),
            size,
        };
    }
}

impl Node for File {
    fn size(&self) -> usize {
        return self.size;
    }
}

fn main() {
    let _input = include_str!("../input.txt");

    println!("--- Part One ---");
    println!("Result: {}", 0);

    println!("--- Part Two ---");
    println!("Result: {}", 0);
}

#[cfg(test)]
mod test {
    use crate::Dir;
    use std::fs;

    #[test]
    fn test_example_case() {
        let file = fs::read_to_string("./src/day07/input.test.txt").expect("file to exist");
        let root = file.parse::<Dir>().expect("root dir to be parseable");
        assert_eq!(root.sum_size_of_dirs_lower_than(), 95437);
    }
}
