use std::io::{Write, Read};
use std::path::{PathBuf, Path};

use regex::Regex;
use structopt::StructOpt;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt;
use std::fs;

use zip;

lazy_static! {
static ref REGEX: Regex = Regex::new(
        r"(?P<dropbox>(?:\w+\s?)+)_(?P<username>\w{8})_attempt_(?P<time>\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2})_(?P<fname>\w+\.\w+)"
).unwrap();
}


#[derive(StructOpt)]
struct Options {
    
    //#[structopt(short="z", long="zip")]
    //zip: bool,

    #[structopt(short="d", long="dst", parse(from_os_str), default_value=".")]
    dst: PathBuf,

    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,

}

struct FileMatch {
    dropbox: String,
    username: String,
    time: String,
    fname: String,
    path: PathBuf,
}

impl fmt::Display for FileMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}-{}", &self.username, &self.fname)
    }
}


struct ZipMatch {
    dropbox: String,
    username: String,
    time: String,
    fname: String,
}

impl fmt::Display for ZipMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}-{}", &self.username, &self.fname)
    }
}



fn process_zip(path: &Path, dst: &Path) {
    let mut zip_archive = zip::ZipArchive::new(
        fs::File::open(&path).unwrap()
    ).unwrap();

    let mut created: HashSet<String> = HashSet::new();

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i).unwrap();
        if let Some(m) = REGEX.captures(&file.name()) {
            let zm = ZipMatch {
                dropbox: String::from(m.name("dropbox").unwrap().as_str()),
                username: String::from(m.name("username").unwrap().as_str()),
                time: String::from(m.name("time").unwrap().as_str()),
                fname: String::from(m.name("fname").unwrap().as_str()),
            };
            //println!("Extracting: {}", &zm);
            if !created.contains(&zm.username) {
                fs::create_dir(&dst.join(&zm.username)).expect(
                    "Could not create directory"
                );
                created.insert(zm.username.to_owned());
            }
            let user_dir = dst.join(&zm.username);
            let mut out_file = fs::File::create(user_dir.join(&zm.fname)).unwrap();

            // There must be a better way of writing from one file object to another
            // without creating a string in memory.
            let mut str_out = String::new();
            file.read_to_string(&mut str_out).ok();
            out_file.write(&str_out.as_bytes()).ok();
        }
    }

}


fn process_dir(path: &Path) -> Vec<FileMatch> {
    let mut rv: Vec<FileMatch> = Vec::new();
    if let Ok(read_dir) = path.read_dir() {
        for item in read_dir {
            if let Ok(p) = item {
                let fn_match: String = match p.path().to_str() {
                    Some(m) => String::from(m),
                    None => {
                        println!("Could not parse filename");
                        continue;
                    }
                };
                if let Some(m) = REGEX.captures(&fn_match) {
                    let fm = FileMatch {
                        dropbox: String::from(m.name("dropbox").unwrap().as_str()),
                        username: String::from(m.name("username").unwrap().as_str()),
                        time: String::from(m.name("time").unwrap().as_str()),
                        fname: String::from(m.name("fname").unwrap().as_str()),
                        path: p.path(),
                    };
                    //println!("Found: {}", &fm);
                    rv.push(fm);
                }
            } else {
                println!("Problem: {:?}", &item);
            }
        }
    } else {
        println!("Failed to read directory {:?}", &path);
    }

    rv
}


fn move_files(dst: &Path, files: Vec<FileMatch>) {
    let mut already: HashSet<String>  = HashSet::new();

    for fnm in &files {
        let mut move_to = dst.join(&fnm.username);
        if !already.contains(&fnm.username) {
            fs::create_dir(&move_to).expect(
                "Could not create directory"
            );
            already.insert(fnm.username.clone());
        }
        move_to.push(&fnm.fname);
        println!("Moving {:?} to {:?}", &fnm.path, &move_to);

        fs::rename(&fnm.path, &move_to).expect("Move failed");
    }

}


fn main() {
    let opts = Options::from_args();
    
    let src = match opts.path {
        Some(t) => t,
        None => PathBuf::from("."),
    };
    //println!("{}", &REGEX.as_str());
    println!("processing {:?}", &src);

    if let Some(ext) = src.extension() {
        if ext == "zip" {
            process_zip(&src, &opts.dst);
        }
    } else {
        let file_matches = process_dir(&src);
        move_files(&opts.dst, file_matches);
    }


    
    
}
