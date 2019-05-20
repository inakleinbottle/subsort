use std::path::{PathBuf, Path};
use std::process::exit;
use regex::Regex;
use structopt::StructOpt;
use lazy_static::lazy_static;

lazy_static! {
static ref REGEX: Regex = Regex::new(
        r"(?P<dropbox>(?:\w+\s)+)_(?P<username>\w{8})_attempt_(?P<time>\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2})_(?P<fname>\w+.m)"
).unwrap();
}


#[derive(StructOpt)]
struct Options {
    
    #[structopt(short="z", long="zip")]
    zip: bool,

    #[structopt(parse(from_os_str))]
    path: Vec<PathBuf>,

}


fn process_dir(path: &Path) {
    if let Ok(read_dir) = path.read_dir() {
        for item in read_dir {
            if let Ok(p) = item {
                println!("Processing: {:?}", &p.path());
            } else {
                println!("Problem: {:?}", &item);
            }
        }
    } else {
        println!("Failed to read directory {:?}", &path);
    }
}

fn main() {
    let mut opts = Options::from_args();
    
    if opts.path.len() == 0 {
        opts.path.push(PathBuf::from("."));
    }
    println!("{}", &REGEX.as_str());

    for path in &opts.path {
        println!("Processing: {:?}", &path);
        process_dir(&path);
    }


}
