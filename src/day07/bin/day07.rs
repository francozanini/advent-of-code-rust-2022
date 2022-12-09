use std::rc::Rc;
use std::str::FromStr;
use std::{i64, isize};

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    sub_dirs: Vec<Dir>,
}

impl Dir {
    fn new(name: &str) -> Self {
        return Dir {
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

        return self
            .sub_dirs
            .iter()
            .flat_map(|sub_dir| sub_dir.size_of_dirs())
            .chain(vec![own_size].into_iter())
            .collect();
    }

    fn sum_size_of_dirs_lower_than_one_hundred_k(&self) -> usize {
        self.size_of_dirs()
            .iter()
            .filter(|size| **size < 100000)
            .sum()
    }

    fn size_of_dir_to_delete_for_update(&self) -> i64 {
        let size_of_dirs = self.size_of_dirs();
        let size_of_dirs_iter = size_of_dirs.iter().map(|size| *size as i64);
        let free_space = 70000000 - self.size() as i64;
        let space_needed = 30000000 - free_space;
        return size_of_dirs_iter
            .filter(|size| size >= &&space_needed)
            .inspect(|size| println!("{}", size))
            .min()
            .expect("to find at least one dir with enough size");
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_dir: String = "/".to_string();
        Ok(s.lines().skip(1).fold(Dir::new("/"), |mut dir, line| {
            return match line.split_once(" ").unwrap() {
                ("$", command) => {
                    if command.starts_with("cd") {
                        current_dir = match command.split_once(" ").unwrap() {
                            (_, "..") => current_dir.rsplit_once("/").unwrap().0.to_string(),
                            (_, to_append) => {
                                let slash_or_space =
                                    if current_dir.ends_with("/") { "" } else { "/" };
                                current_dir.clone() + slash_or_space + to_append
                            }
                        };
                    }
                    dir
                }
                ("dir", dir_name) => {
                    let slash_or_space = if current_dir.ends_with("/") { "" } else { "/" };
                    dir.add_dir(
                        current_dir.as_str(),
                        (current_dir.clone() + slash_or_space + dir_name).as_str(),
                    );
                    dir
                }
                (file_size, file_name) => {
                    dir.add_file(
                        current_dir.as_str(),
                        file_name,
                        file_size
                            .parse::<usize>()
                            .expect("first argument to be a num"),
                    );
                    dir
                }
            };
        }))
    }
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
    use std::time::Instant;

    #[test]
    fn part_one_example() {
        let file = fs::read_to_string("./src/day07/input.test.txt").expect("file to exist");
        let root = file.parse::<Dir>().expect("root dir to be parseable");
        assert_eq!(root.sum_size_of_dirs_lower_than_one_hundred_k(), 95437);
    }

    #[test]
    fn part_one_prod() {
        let file = fs::read_to_string("./src/day07/input.txt").expect("file to exist");
        let root = file.parse::<Dir>().expect("root dir to be parseable");
        assert_eq!(root.sum_size_of_dirs_lower_than_one_hundred_k(), 1348005);
    }

    #[test]
    fn part_two_example() {
        let file = fs::read_to_string("./src/day07/input.test.txt").expect("file to exist");
        let root = file.parse::<Dir>().expect("root dir to be parseable");
        assert_eq!(root.size_of_dir_to_delete_for_update(), 24933642);
    }

    #[test]
    fn part_two_prod() {
        let file = fs::read_to_string("./src/day07/input.txt").expect("file to exist");
        let root = file.parse::<Dir>().expect("root dir to be parseable");
        assert_eq!(root.size_of_dir_to_delete_for_update(), 12785886);
    }

    #[test]
    fn bigboy() {
        let file = fs::read_to_string("./src/day07/bigboy.txt").expect("file to exist");
        let now = Instant::now();
        let root = file.parse::<Dir>().expect("root dir to be parseable");
        println!("Parsing solved after {} seconds", now.elapsed().as_secs());
        let first_answer = root.sum_size_of_dirs_lower_than_one_hundred_k();
        println!(
            "First problem solved after {} seconds",
            now.elapsed().as_secs()
        );
        let second_answer = root.size_of_dir_to_delete_for_update();
        println!(
            "Second problem solved after {} seconds",
            now.elapsed().as_secs()
        );
        assert_eq!(first_answer, 2414990429);
        assert_eq!(second_answer, 170301725);
    }
}
