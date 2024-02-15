use log::{debug, trace};
use nmuidi::nmuidi::Cleaner;
use std::{env, time::Instant};

fn main() {
    if env::args().any(|arg| arg == "-h") {
        println!("Usage: nmuidi <dir> [dirs...] (-y)");
        return;
    }
    
    if env::args().last().unwrap() == "-y" {
        println!("Deleting without confirmation...");
        clean();
    } else {
        println!("Are you sure you want to delete the folder and everything inside of it? (y/N)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            clean();
        } else {
            println!("Exiting...");
            return;
        }
    }
}

fn clean() {
    pretty_env_logger::init();

    let mut directory_timings = Vec::new();
    let start_time = Instant::now();
    for dir in env::args().skip(1) {
        if dir == "-y" {
            continue;
        }
        println!("Cleaning {dir}");
        let start = Instant::now();

        Cleaner::new(&dir).clean();
        directory_timings.push((dir, start.elapsed()));
    }

    let elapsed_time = start_time.elapsed();
    debug!("Total time: {:.2?}", elapsed_time);
    debug!("Directory timings:");
    for (dir, time_spent) in directory_timings {
        debug!("  dir {dir} took {:.2?}", time_spent);
    }
    trace!("Done.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use jwalk::WalkDir;
    use std::fs;

    #[test]
    fn test_nested() {
        fs::create_dir_all("tmp/nested/dir1").unwrap();
        fs::write("tmp/nested/dir1/file1.txt", "File 1 content").unwrap();

        fs::create_dir_all("tmp/nested/dir1/dir2").unwrap();
        fs::write("tmp/nested/dir1/dir2/file2.txt", "File 2 content").unwrap();

        fs::create_dir_all("tmp/nested/dir1/dir2/dir3").unwrap();
        fs::write("tmp/nested/dir1/dir2/dir3/file3.txt", "File 3 content").unwrap();

        Cleaner::new("tmp/nested").clean();

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

        Cleaner::new("tmp/dirs").clean();

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

        Cleaner::new("tmp/files").clean();

        let num_files = WalkDir::new("tmp/files")
            .skip_hidden(false)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        assert_eq!(num_files, 1);
    }
}
