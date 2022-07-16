//todo@ remove unused imports
use clap::Parser;
use lazy_regex::regex;
use once_cell::sync::OnceCell;
use std::{
    env::var_os,
    fs::{metadata, read_dir, File},
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
    sync::Mutex,
    thread,
};

static mut DID_PRINT: bool = false;
static mut COLOR: bool = true;
static mut SINGLE_THREAD: bool = false;

static PRINT_LOCK: OnceCell<Mutex<bool>> = OnceCell::new();

/// static mut wrapper
macro_rules! static_var {
    ($var:ident) => {
        unsafe { $var }
    };

    ($var:ident = $value:expr) => {
        unsafe { $var = $value }
    };
}

macro_rules! color {

    (black) => {
        color!(@internal "0")
    };

    (red) => {
        color!(@internal "1")
    };

    (green) => {
        color!(@internal "2")
    };

    (yellow) => {
        color!(@internal "3")
    };

    (blue) => {
        color!(@internal "4")
    };

    (magenta) => {
        color!(@internal "5")
    };

    (cyan) => {
        color!(@internal "6")
    };

    (white) => {
        color!(@internal "7")
    };

    (reset) => {
        if static_var!(COLOR) {
            "\x1B[0m"
        } else {
            ""
        }
    };

    (@internal $code:literal) => {
        if static_var!(COLOR) {
            concat!("\x1B[0m\x1B[1;3", $code, "m")
        } else {
            ""
        }
    }
}

#[derive(Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    Todo(Todo),
}

#[derive(clap::Args)]
#[clap(
    after_help = "To disable colored output, execute with environment variable NO_COLOR\n\n",
    about,
    version
)]
struct Todo {
    #[clap(
        short,
        long,
        help = "Either a file or a directory",
        default_value = "./src"
    )]
    path: String,

    #[clap(short, long, help = "Limit CPU usage to a single OS thread")]
    single_thread: bool,
}

fn walk_dir<P: AsRef<Path>>(path: P) {
    if let Ok(metadata) = metadata(&path) {
        if metadata.is_dir() {
            if let Ok(dir) = read_dir(&path) {
                let mut handles = Vec::new();
                for entry in dir.flatten() {
                    if static_var!(SINGLE_THREAD) {
                        walk_dir(entry.path());
                        continue;
                    }
                    handles.push(thread::spawn(move || walk_dir(entry.path())));
                }

                if !static_var!(SINGLE_THREAD) {
                    for handle in handles {
                        if handle.join().is_err() {
                            eprintln!("[FATAL]: thread panicked");
                            exit(255);
                        }
                    }
                }
            }
        } else if metadata.is_file() {
            if let Ok(file) = File::open(&path) {
                let mut file = BufReader::new(file);
                let mut line = String::new();
                let mut counter = 1;

                while let Ok(bytes_read) = file.read_line(&mut line) {
                    if let Some(captures) =
                        regex!(r"//[\s\S]*@[Tt][Oo][Dd][Oo][\s:]*(?P<comment>.*)").captures(&line)
                    {
                        static_var!(DID_PRINT = true);

                        let _lock = PRINT_LOCK.get().unwrap().lock();

                        println!("{}---{}", color!(blue), color!(white),);

                        println!(
                            "\n{}File:{} {}",
                            color!(yellow),
                            color!(white),
                            path.as_ref().display(),
                        );

                        println!("{}Line:{} {}", color!(yellow), color!(white), counter,);

                        let comment = &captures["comment"].trim();

                        if !comment.is_empty() {
                            println!("{}Comment:{}\n\t{comment}", color!(yellow), color!(white),);
                        }

                        println!();
                    }

                    if bytes_read == 0 {
                        break;
                    }
                    counter += 1;
                    line.clear();
                }
            }
        }
    }
}

fn main() {
    static_var!(COLOR = var_os("NO_COLOR").is_none());

    PRINT_LOCK.set(Mutex::new(false)).unwrap();

    let Cargo::Todo(args) = Cargo::parse();

    static_var!(SINGLE_THREAD = args.single_thread);

    walk_dir(&args.path);

    if static_var!(DID_PRINT) {
        println!("{}---{}", color!(blue), color!(reset),);
        exit(1)
    } else {
        println!("{}No todo(s) found{}", color!(green), color!(reset),);
    }
}
