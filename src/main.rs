use clap::Parser;
use std::fs;

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

fn create_tar_ball(source: fs::File)

fn create_copy(t: PayloadType) {
    // Create temp dir
    fs::create_dir("transfer_temp").expect("Directory creations failed.");

    match t {
        PayloadType::File(path) => fs::copy(path, "transfer_temp").expect("File copy failed."),
        PayloadType::Folder(path) => {
            let 
        }
    };

}

fn main() {
    let args = Args::parse();
    let t = get_payload_type(args.filename);
}
