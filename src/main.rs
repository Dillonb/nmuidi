use std::{fs, env, time::SystemTime};

use itertools::Itertools;
use jwalk::{ WalkDir, Error, rayon::prelude::{ParallelBridge, ParallelIterator, IntoParallelRefIterator} };

fn clean(dir_to_clean : String) -> Result<(), Error> {
    let threads = num_cpus::get() * 100;
    println!("Cleaning with {} threads.", threads);
    let start_time = SystemTime::now();

    let mut dirs : Vec<(std::path::PathBuf, usize)> = WalkDir::new(dir_to_clean)
        .skip_hidden(false)
        .parallelism(jwalk::Parallelism::RayonNewPool(threads))
        .into_iter()
        .par_bridge()
        .flat_map(|entry| {
            let mut dirs = Vec::new();
            match entry {
                Ok(e) => {
                    let f_type = e.file_type;
                    let path = e.path();
                    let metadata = e.metadata().unwrap();

                    let mut perm = metadata.permissions();
                    if perm.readonly() {
                        perm.set_readonly(false);
                        match fs::set_permissions(&path, perm) {
                            Ok(()) => (),
                            Err(error) => println!("Error making {} not read only: {}", path.display(), error),
                        }
                    }
                    if f_type.is_file() || f_type.is_symlink() {
                        match fs::remove_file(&path) {
                            Ok(()) => (),
                            Err(error) => println!("Failed to remove file {}: {}", path.display(), error),
                        }
                    } else if f_type.is_dir() {
                        dirs.push((path, e.depth));
                    }
                }
                Err(error) => println!("Error processing entry: {}", error),
            }
            return dirs;
        })
        .collect();
    let files_done = SystemTime::now();
    println!("Done cleaning files, took {} seconds. Starting on dirs", files_done.duration_since(start_time).unwrap().as_secs());
    dirs.sort_by(|a, b| {
        b.1.cmp(&a.1)
    });
    let sorting_done = SystemTime::now();
    println!("Done sorting, took {} seconds. Starting to delete directories.", sorting_done.duration_since(files_done).unwrap().as_secs());

    let dirs_by_depth = dirs.iter().group_by(|x| x.1);
    for (_, dirs) in &dirs_by_depth {
        dirs.map(|x| &x.0).collect::<Vec<_>>()
            .par_iter()
            .for_each(|dir| {
                match fs::remove_dir_all(dir.as_path()) {
                    Err(error) => println!("Error removing directory {}: {}", dir.display(), error),
                    Ok(()) => ()
                }
            });
    }

    let everything_done = SystemTime::now();
    println!("Done deleting directories, took {} seconds. Entire process took {} seconds.", 
        everything_done.duration_since(sorting_done).unwrap().as_secs(),
        everything_done.duration_since(start_time).unwrap().as_secs(),
    );

    return Ok(());
}

fn main() {
    for dir in env::args().skip(1) {
        println!("Cleaning {}", dir);
        let _ = clean(dir);
    }
    println!("Done.");
}