use std::process::Command;
use std::env;
use std::fs::{File, read_dir};
use std::path::Path;
use std::collections::HashMap;
use std::io::prelude::*;

static FEATURES: &'static [&'static str] = &["PROFILE"];

fn main() {
    unifdef_walk(detect_features());
}

fn unifdef_file(file: &Path, features: &HashMap<String, bool>) {
    Command::new("unifdef")
            .arg("-t")
            .args(&features.iter()
                           .map(|(s, &b)| format!("-{}{}", if b { "D" } else { "U" },
                                                          s))
                           .collect::<Vec<_>>())
            .arg(file)
            .output()
            .and_then(|o| File::create(file.with_file_name(file.file_stem().unwrap()))
                               .and_then(|mut f| f.write_all(&o.stdout)))
            .unwrap();
}

fn unifdef_walk(features: HashMap<String, bool>) {
    for de in read_dir("src/").unwrap()
                             .filter_map(Result::ok)
                             .filter(|de| de.path().extension().map(|s| s == "pre") == Some(true)) {
        unifdef_file(&de.path(), &features);
    }
}

fn detect_features() -> HashMap<String, bool> {
    FEATURES.iter()
            .map(|f| ((*f).to_owned(), env::var(format!("CARGO_FEATURE_{}", f)).is_ok()))
            .collect()
}

