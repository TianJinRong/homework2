use std::path::PathBuf;
use structopt::StructOpt;
use std::io::Result;
use std::fs;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust_fuind", about = "A command line utility for searching for files with regexes")]
struct CLI {
    #[structopt(short, long, required = true, help = "List of directories to search in.")]
    dirs: Vec<PathBuf>,
    /// List of patterns to use.
    #[structopt(short, long, required = true)]
    patterns: Vec<String>,
    /// Write results to output file instead of stdout.
    #[structopt(short, long, required = false, default_value = "./")]
    output: PathBuf,
    /// Match files above size <size>.
    #[structopt(short, long, required = false, default_value = "0")]
    size: u64
}

struct MyFile {
    name: String,
    dir_in: String,
    size_bytes: u64
}

impl MyFile {
    fn from_path(&self, path: PathBuf) -> Result<Self> {
        let result = Self {
            name: path.file_name(),
            dir_in: path.parent().to_string_lossy(),
            size_bytes: fs::metadata(path)?.len()
        };
        Ok(result)
    }
}

#[test]
fn test_from_path() {
    let test: PathBuf = PathBuf::from(".src/");
    let test: MyFile = MyFile::from(test);
    assert_eq!(MyFile{ name: String::from(".src/main.rs"), dir_in: String::from(".src/"), size_bytes: 100}, test);
}

fn get_files(paths: Vec<PathBuf>) -> Vec<MyFile> {
    let sub_pathbufs: Vec<PathBuf> = Vec::new();
    let my_files: Vec<MyFile> = Vec::new();
    // Check if the path is dir
    for pathbuf in &paths {
        print!("{} is dir? {}", pathbuf.to_string_lossy(), pathbuf.is_dir());

        if pathbuf.is_dir() == false {
            print!("{} is not dir.", pathbuf.to_string_lossy());
            my_files.push(MyFile::from_path(pathbuf));
            continue;
        }

        // Iterate through the dir.
        print!("Reading dir {} ...\n", pathbuf.to_string_lossy());
        let subpaths = fs::read_dir(pathbuf).unwrap();
        for subpath in subpaths {
            let path: PathBuf = subpath.unwrap().path();
            if path.is_dir() {
                print!("{} is dir.", path.to_string_lossy());
                sub_pathbufs.push(path);
            } else {
                my_files.push(MyFile::from_path(path));
                print!("{} is not dir.", path.to_string_lossy());
            }
        }
    }

    let sub_files: Vec<MyFile> = get_files(sub_pathbufs);
    for sub_file in sub_files {
        my_files.push(sub_file);
    }
    my_files
}

fn main() {
    // Read arguments from command
    let args = CLI::from_args();
    println!("{:?}", args);
    // Find file
    let dirs: Vec<PathBuf> = args.dirs;
    let patterns: Vec<String> = args.patterns;
    let output: PathBuf = args.output;
    let size: u64 = args.size;
    let my_files: Vec<MyFile> = get_files(dirs);

    // Output result
    println!("{}", my_files);
}
