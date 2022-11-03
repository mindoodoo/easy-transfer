#![feature(decl_macro)] // Enable experimental rocket api

use std::{fs, process};
use clap::Parser;
use flate2::{self, write};
use tar;


// ─── Arg Struct For Clap ─────────────────────────────────────────────────────────────────────────
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   filename: String,
}

// ─── Payload Stuff ───────────────────────────────────────────────────────────────────────────────
enum PayloadType {
    File(String),
    Folder(String)
}

fn get_payload_type(filepath: String) -> Option<PayloadType> {
    let attr = fs::metadata(&filepath);

    match attr {
        Ok(attr) => {
            if attr.is_dir() {
                Some(PayloadType::Folder(filepath))
            } else if attr.is_file() {
                Some(PayloadType::File(filepath))
            } else {
                None
            }
        },
        Err(_) => None
    }
}

fn create_tar_ball(source: &str, output: &str) {
    let file = fs::File::create(output).expect("File creation failed.");
    let encoder = write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar_ball = tar::Builder::new(encoder);
    tar_ball.append_dir_all(".", source.clone()).expect("Adding files to tar ball failed.");
}

fn prep_files(t: PayloadType) {
    if let PayloadType::Folder(path) = t {
            create_tar_ball(&path, "archive.tar.gz");
    };
}

// ─── Web Server ──────────────────────────────────────────────────────────────────────────────────
#[rocket::get("/")]
pub fn download() {
    
}


fn main() {
    let args = Args::parse();
    let t = get_payload_type(args.filename);

    // ─── Prepare Files ───────────────────────────────────────────────────────────────────────
    match t {
        None => {
            println!("Invalid input file, exiting now...");
            process::exit(1);
        },
        Some(x) => {
            prep_files(x);
        },
    };

    // ─── Run Web Server ──────────────────────────────────────────────────────────────────────
    rocket::ignite().mount("/", rocket::routes![download]).launch();    
}

// fn main() {
//     let file = fs::File::create("foo.tar").unwrap();
//     let mut a = tar::Builder::new(file);

//     a.append_dir_all(".", "src").unwrap();
//     // a.append_file("file2.txt", &mut fs::File::open("file3.txt").unwrap()).unwrap();
// }
