
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;
use std::error::Error;
use std::io;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust_find", about = "A command line utility for searching for files with regexes")]
pub struct Config {
    #[structopt(short, long, required = true, help = "List of directories to search in.")]
    pub dirs: Vec<PathBuf>,
    /// List of patterns to use.
    #[structopt(short, long, required = true)]
    pub patterns: Vec<String>,
    /// Write results to output file instead of stdout.
    #[structopt(short, long, required = false, default_value = "./")]
    pub output: PathBuf,
    /// Match files above size <size>.
    #[structopt(short, long, required = false, default_value = "0")]
    pub size: u64
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // let dirs: Vec<PathBuf> = args.dirs;
    // let patterns: Vec<String> = args.patterns;
    // let output: PathBuf = args.output;
    // let size: u64 = args.size;
    let contents = fs::read_to_string("./main.rs")?;
    println!("With text:\n{}", contents);
    Ok(())
}

pub struct MyFile {
    name: String,
    dir_in: String,
    size_bytes: u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_pathbuf_exists() {
        let path: PathBuf = PathBuf::from(r"./");
        assert_eq!(true, path.is_dir());
    }

    #[test]
    fn test_read_dir() -> io::Result<()> {
        // 测试读取某个目录下的文件
        let path: PathBuf = PathBuf::from(r"./");
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let sub_path = entry.path();
            if sub_path.is_dir() {
                println!("{} is dir? {}", sub_path.to_string_lossy(), sub_path.is_dir());
            } else {
                println!("{} is file.", sub_path.to_string_lossy());
            }
        }
        Ok(())
    }

    #[test]
    fn test_read_files() -> io::Result<()> {
        // 测试递归读取某目录下的所有文件路径
        Ok(())
    }
}

// impl MyFile {
//     pub fn from_path(&self, path: PathBuf) -> Result<Self> {
//         let result = Self {
//             name: path.file_name(),
//             dir_in: path.parent().to_string_lossy(),
//             size_bytes: fs::metadata(path)?.len()
//         };
//         Ok(result)
//     }
// }
// let args: Vec<String> = env::args().collect();
// let config = Config::new(&args);


// #[test]
// fn test_from_path() {
//     let test: PathBuf = PathBuf::from(".src/");
//     let test: MyFile = MyFile::from(test);
//     assert_eq!(MyFile{ name: String::from(".src/main.rs"), dir_in: String::from(".src/"), size_bytes: 100}, test);
// }

// fn get_files(paths: Vec<PathBuf>) -> Vec<MyFile> {
//     let sub_pathbufs: Vec<PathBuf> = Vec::new();
//     let my_files: Vec<MyFile> = Vec::new();
//     // Check if the path is dir
//     for pathbuf in &paths {
//         print!("{} is dir? {}", pathbuf.to_string_lossy(), pathbuf.is_dir());

//         if pathbuf.is_dir() == false {
//             print!("{} is not dir.", pathbuf.to_string_lossy());
//             my_files.push(MyFile::from_path(pathbuf));
//             continue;
//         }

//         // Iterate through the dir.
//         print!("Reading dir {} ...\n", pathbuf.to_string_lossy());
//         let subpaths = fs::read_dir(pathbuf).unwrap();
//         for subpath in subpaths {
//             let path: PathBuf = subpath.unwrap().path();
//             if path.is_dir() {
//                 print!("{} is dir.", path.to_string_lossy());
//                 sub_pathbufs.push(path);
//             } else {
//                 my_files.push(MyFile::from_path(path));
//                 print!("{} is not dir.", path.to_string_lossy());
//             }
//         }
//     }

//     let sub_files: Vec<MyFile> = get_files(sub_pathbufs);
//     for sub_file in sub_files {
//         my_files.push(sub_file);
//     }
//     my_files
// }







// 返回错误
// 写法1: panic
// if args.len() < 3 {
//   panic!("not enough arguments!");
// }
// // 写法2: 返回 Result
// // 如果没有发生错误，就直接返回结果
// // 如果有错误，就走 unwrap_or_else 里的逻辑
// fn new(args: &[String]) -> Result<Config, &'static str> {
//   if args.len() < 3 {
//     return Err("not enough arguments!");
//   }
//   let query = args[1].clone();
//   let filename = &args[2].clone();
//   Ok(Config{ query, filename })
// }
// let config = Config::new(&args).unwrap_or_else(|err| {
//   println!("Problem parsing arguments: {}", err);
//   process::exit(1); // 退出程序
// });
// // 写法3: 直接返回错误
// use std::error::Error;
// use std::fs;
// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//   // ?的作用就是报错了直接返回这个错误
//   let contents = fs::read_to_string(config.filename)?;
//   println!("With text:\n{}", contents);
//   Ok(())
// }
// if let Err(e) = xxx::run(config) {
//   println!("Application err: {}", e);
//   process::exit(1);
// }



// 最佳实践
// main.rs + lib.rs
// lib.rs 里有 run, Config
// main.rs
// use xxx::Config;
// use std::end;
// use std::process;
// fn main() {
//   let config = Config::new(env::args()).unwrap_or_else(|err| {
//     println!("Problem parsing arg: {}", err);
//     process::exit(1);
//   });
//   if let Err(e) = xxx::run(config) {
//     println!("Application err: {}", e);
//     process::exit(1);
//   }
// }
// // lib.rs
// use std::error::Error;
// use std::fs;
// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//   // ?的作用就是报错了直接返回这个错误
//   let contents = fs::read_to_string(config.filename)?;
//   println!("With text:\n{}", contents);
//   Ok(())
// }
// pub struct Config {
//   query: String,
//   filename: String,
// }
// impl Config {
//   fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
//   if args.len() < 3 {
//     return Err("not enough arguments!");
//   }
//   args.next(); // 第一个值是命令本身
//   let query = match args.next() {
//     Some(arg) => arg,
//     None => return Err("error1")
//   };
//   let filename = match args.next() {
//     Some(arg) => arg,
//     None => return Err("error2")
//   };
//   Ok(Config{ query, filename })
//   }
// }
