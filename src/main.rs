use audiotags::Tag;
use clap::Parser;
use std::path::Path;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    directory: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.directory);
    println!("Directory '{}'", path.canonicalize()?.display());

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let tag = Tag::new().read_from_path(entry.path());
        if let Ok(t) = tag {
            println!(
                "[Artist]/[Year] - [Album]/[Track] - [Title] = {}/{} - {}/{} - {}",
                t.artist().unwrap(),
                t.year().unwrap(),
                t.album_title().unwrap(),
                t.track_number().unwrap(),
                t.title().unwrap()
            );
        }
    }

    Ok(())
}
