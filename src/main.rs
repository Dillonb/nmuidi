use std::{env, fs, time::Instant};

use itertools::Itertools;
use jwalk::{
    rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    WalkDir,
};

fn clean(dir_to_clean: &str) {
    let threads = num_cpus::get() * 100;
    println!("Cleaning with {threads} threads.");
    let start_time = Instant::now();

    let mut dirs: Vec<(std::path::PathBuf, usize)> = WalkDir::new(dir_to_clean)
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
                        // This is a UNIX concern, not a prob on Windows
                        #[allow(clippy::permissions_set_readonly_false)]
                        perm.set_readonly(false);
                        match fs::set_permissions(&path, perm) {
                            Ok(()) => (),
                            Err(error) => {
                                println!(
                                    "Error making {} not read only: {}",
                                    path.display(),
                                    error
                                );
                            }
                        }
                    }
                    if f_type.is_file() || f_type.is_symlink() {
                        match fs::remove_file(&path) {
                            Ok(()) => (),
                            Err(error) => {
                                println!("Failed to remove file {}: {error}", path.display());
                            }
                        }
                    } else if f_type.is_dir() {
                        dirs.push((path, e.depth));
                    }
                }
                Err(error) => println!("Error processing entry: {error}"),
            }
            dirs
        })
        .collect();
    let files_done = Instant::now();
    println!(
        "Done cleaning files, took {} seconds. Starting on dirs",
        start_time.elapsed().as_secs_f32()
    );
    dirs.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "Done sorting, took {} seconds. Starting to delete directories.",
        files_done.elapsed().as_secs_f32()
    );

    let dirs_by_depth = dirs.iter().group_by(|x| x.1);
    for (_, level) in &dirs_by_depth {
        level
            .collect::<Vec<_>>()
            .par_iter()
            .map(|(dir, _group)| dir)
            .for_each(|dir| {
                if let Err(e) = fs::remove_dir_all(dir.as_path()) {
                    println!("Error removing directory {}: {e}", dir.display());
                }
            });
    }
}

fn main() {
    let mut directory_timings = Vec::new();
    let start_time = Instant::now();
    for dir in env::args().skip(1) {
        println!("Cleaning {dir}");
        let start = Instant::now();
        clean(&dir);
        directory_timings.push((dir, start.elapsed()));
    }

    let elapsed_time = start_time.elapsed();
    println!("Total time: {}s", elapsed_time.as_secs_f32());
    println!("Directory timings:");
    for (dir, time_spent) in directory_timings {
        println!("  dir {dir} took {}s", time_spent.as_secs_f32());
    }
    println!("Done.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested() {
        fs::create_dir_all("tmp/nested/dir1").unwrap();
        fs::write("tmp/nested/dir1/file1.txt", "File 1 content").unwrap();

        fs::create_dir_all("tmp/nested/dir1/dir2").unwrap();
        fs::write("tmp/nested/dir1/dir2/file2.txt", "File 2 content").unwrap();

        fs::create_dir_all("tmp/nested/dir1/dir2/dir3").unwrap();
        fs::write("tmp/nested/dir1/dir2/dir3/file3.txt", "File 3 content").unwrap();

        clean("tmp/nested");

        let num_files = WalkDir::new("tmp/nested")
            .skip_hidden(false)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        assert_eq!(num_files, 1);
    }

    #[test]
    fn test_dirs() {
        fs::create_dir_all("tmp/dirs/dir1").unwrap();
        fs::create_dir_all("tmp/dirs/dir1a").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2a").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2/dir3").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2/dir3a").unwrap();

        clean("tmp/dirs");

        let num_files = WalkDir::new("tmp/dirs")
            .skip_hidden(false)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        assert_eq!(num_files, 1);
    }

    #[test]
    fn test_files() {
        fs::create_dir_all("tmp/files").unwrap();
        fs::write("tmp/files/file1.txt", "File 1 content").unwrap();
        fs::write("tmp/files/file2.txt", "File 2 content").unwrap();
        fs::write("tmp/files/file3.txt", "File 3 content").unwrap();
        fs::write("tmp/files/file4.txt", "File 4 content").unwrap();
        fs::write("tmp/files/file5.txt", "File 5 content").unwrap();
        fs::write("tmp/files/file6.txt", "File 6 content").unwrap();

        clean("tmp/files");

        let num_files = WalkDir::new("tmp/files")
            .skip_hidden(false)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        assert_eq!(num_files, 1);
    }
}
