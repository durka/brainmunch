extern crate foreman;

use std::collections::HashSet;
use std::fs::{File, read_dir};
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use std::io::prelude::*;

fn main() {
    let tic = Instant::now();
    unifdef_walk(&foreman::features().unwrap());
    let toc = Instant::now() - tic;
    foreman::warning(&format!("preprocessing: {:.3}s",
                              toc.as_secs() as f64 + (toc.subsec_nanos() as f64 / 1e9)));
}

fn unifdef_file(file: &Path, features: &[String]) {
    foreman::warning(&format!("unifdef {}", file.display()));
    let syms = String::from_utf8(
        Command::new("unifdef")
            .arg("-st")
            .arg(file)
            .output().unwrap()
            .stdout).unwrap()
                    .split("\n")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect::<HashSet<_>>();
    Command::new("unifdef")
            .arg("-t")
            .args(&syms.iter()
                       .map(|s| format!("-{}{}", if features.contains(&s) { "D" } else { "U" },
                                                 s))
                       .collect::<Vec<_>>())
            .arg(file)
            .output()
            .and_then(|o| File::create(file.with_file_name(file.file_name().unwrap()
                                                               .to_string_lossy()
                                                               .replace(".pre.rs", ".rs")))
                               .and_then(|mut f| f.write_all(&o.stdout)))
            .unwrap();
}

fn unifdef_walk(features: &[String]) {
    for de in read_dir("src/").unwrap()
                              .filter_map(Result::ok)
                              .filter(|de| de.path()
                                             .file_name()
                                             .map(|s| s.to_string_lossy()
                                                       .ends_with(".pre.rs"))
                                             == Some(true)) {
        unifdef_file(&de.path(), features);
    }
}

