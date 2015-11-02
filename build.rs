use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    Command::new("unifdef")
            .arg("-t")
            .arg("-UPROFILE")
            .arg("src/main.rs.pre")
            .output()
            .and_then(|o| File::create("src/main.rs")
                      .and_then(|mut f| f.write_all(&o.stdout)))
            .unwrap();
}

