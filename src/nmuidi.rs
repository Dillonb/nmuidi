use std::{fs, path::PathBuf};

use itertools::Itertools;

use jwalk::{
    rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    WalkDir,
};

use log::error;

pub struct Cleaner {
    path: PathBuf,
    dirs: Vec<(PathBuf, usize)>, // (path, depth)
    threads: usize,
}

impl Cleaner {
    pub fn new<T>(path: T) -> Self
    where
        std::path::PathBuf: std::convert::From<T>,
    {
        Self {
            path: path.into(),
            dirs: Vec::new(),
            threads: num_cpus::get() * 100,
        }
    }

    pub fn clean(&mut self) {
        self.remove_files();
        self.remove_dirs();
    }

    fn remove_dirs(&mut self) {
        let dirs_by_depth = self.dirs.iter().group_by(|x| x.1);
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

    fn remove_files(&mut self) {
        let mut dirs: Vec<(std::path::PathBuf, usize)> = WalkDir::new(&self.path)
            .skip_hidden(false)
            .parallelism(jwalk::Parallelism::RayonNewPool(self.threads))
            .into_iter()
            .par_bridge()
            .flat_map(|entry| {
                match entry {
                    Ok(entry) => {
                        let f_type = entry.file_type;
                        let path = entry.path();
                        let metadata = entry.metadata().unwrap();

                        let mut perm = metadata.permissions();
                        if perm.readonly() {
                            // This is a UNIX concern, not a prob on Windows
                            #[allow(clippy::permissions_set_readonly_false)]
                            perm.set_readonly(false);
                            fs::set_permissions(&path, perm).unwrap_or_else(|e| {
                                error!("Error making {} write-accessable: {e}", path.display());
                            });
                        }
                        if f_type.is_file() || f_type.is_symlink() {
                            fs::remove_file(&path).unwrap_or_else(|e| {
                                error!("Failed to remove file {}: {e}", path.display());
                            });
                        } else if f_type.is_dir() {
                            return Some((path, entry.depth));
                        }
                    }
                    Err(error) => error!("Error processing directory entry: {error}"),
                }
                None
            })
            .collect();
        dirs.sort_by(|a, b| b.1.cmp(&a.1)); // Note reverse sort
        self.dirs = dirs;
    }
}
