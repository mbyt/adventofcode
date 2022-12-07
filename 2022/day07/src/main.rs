use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Dir {
    files: Vec<(String, i64)>,
    files_size: i64,
}

fn main() {
    let limit = 100_000;

    // test
    let example = get_example_input();
    let size = sum_folder_size_flow_limt_from_raw_output(&example, limit);
    assert_eq!(size, 95437);

    // main
    let path = Path::new("input");
    assert!(path.exists());
    
    let contents = fs::read_to_string(path).unwrap().trim().to_owned();
    let size = sum_folder_size_flow_limt_from_raw_output(&contents, limit);

    println!("Total size of folders below 100000: {size}");
    // regression test for refactoring
    assert_eq!(size, 1783610);
}

fn sum_folder_size_flow_limt_from_raw_output(example: &str, limit: i64) -> i64 {
    let fs = fs_from_raw_output(example);
    let folders_and_size = du_dirs(&fs);
    let total_size_of_dirs_below_size_100_000 =
        sum_folder_size_below_limit(&folders_and_size, limit);
    total_size_of_dirs_below_size_100_000
}

fn get_example_input() -> String {
    let example = r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "#
    .trim();
    example.to_owned()
}

fn sum_folder_size_below_limit(folders_and_size: &Vec<(String, i64)>, limit: i64) -> i64 {
    // find all files below limit=100000 and sum them
    let total_size_of_dirs_below_size_100_000 = folders_and_size
        .iter()
        .filter(|(_s, size)| *size <= limit)
        .map(|(_s, size)| size)
        .fold(0, |acc, x| acc + x);
    total_size_of_dirs_below_size_100_000
}

fn du_dirs(fs: &HashMap<String, Dir>) -> Vec<(String, i64)> {
    // du
    let mut folders_and_size: Vec<(String, i64)> = Vec::new();
    for (parent_dir, prop_parent_dir) in fs {
        let mut total_size = prop_parent_dir.files_size;
        for (dir_name, prop_dir_name) in fs {
            if dir_name != parent_dir && dir_name.starts_with(parent_dir) {
                // child dir
                total_size += prop_dir_name.files_size;
            }
        }
        folders_and_size.push((parent_dir.to_owned(), total_size));
    }
    folders_and_size
}

fn fs_from_raw_output(example: &str) -> HashMap<String, Dir> {
    let mut fs: HashMap<String, Dir> = HashMap::new();
    let mut cwd = String::new();
    let mut ls = false;
    for line_with_space in example.lines() {
        let line = line_with_space.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        // command mode
        if parts[0] == "$" {
            let cmd = parts[1];
            match cmd {
                "cd" if parts[2] == "/" => {
                    cwd = "/".to_owned();
                    ls = false;
                }
                "cd" if parts[2] == ".." => {
                    let mut cwd_tmp_vec: Vec<&str> = cwd.split("/").collect();
                    cwd_tmp_vec.pop(); // remove one directory from the stack
                    cwd = cwd_tmp_vec.join("/");
                    ls = false;
                }
                "cd" if cwd == "/" => {
                    cwd = format!("/{}", parts[2]);
                    ls = false;
                }
                "cd" => {
                    cwd = format!("{}/{}", cwd.clone(), parts[2]);
                    ls = false;
                }
                "ls" => {
                    fs.insert(
                        cwd.clone(),
                        Dir {
                            files: Vec::new(),
                            files_size: 0,
                        },
                    );
                    ls = true;
                }
                _ => panic!("ERROR: Unknown command {cmd} in line {line}"),
            }
        } else if ls == true {
            if parts[0] == "dir" {
                // the stored directory are represented by by their path
                // directories without files cannot be seen
            } else {
                let (size_str, filename) = (parts[0], parts[1]);
                let size: i64 = size_str.parse().unwrap();
                let Some(dir) = fs.get_mut(&cwd) else {
                    panic!("cwd={} not contained in fs={:?}", cwd, &fs);
                };
                (*dir).files_size += size;
                (*dir).files.push((filename.to_owned(), size));
            }
        }
    }
    fs
}
