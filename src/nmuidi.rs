use std::{fs, path::PathBuf};

use itertools::Itertools;
use jwalk::{
    rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    WalkDir,
};
use log::error;

/// `Cleaner` is a lazily executed framework for nmuidi.
/// # Examples
/// ```
/// use nmuidi::prelude::*;
/// let cleaner = Cleaner::new("some/path").clean();
/// ```
pub struct Cleaner {
    path: PathBuf,
    dirs: Vec<(PathBuf, usize)>, // (path, depth)
    threads: usize,
}

impl Cleaner {
    /// Form a new `Cleaner` stuct, does not execute anything until `.clean()` is called
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

    /// Perform the deletion of the selected directory
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

    fn is_reparse_point(meta: &std::fs::Metadata) -> bool {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
        meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
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
                        let path = entry.path();

                        // Get metdata while avoiding walking into junctions
                        let metadata = match fs::symlink_metadata(&path) {
                            Ok(m) => m,
                            Err(e) => {
                                error!("Failed to get metadata for {}: {e}", path.display());
                                return None;
                            }
                        };

                        let f_type = metadata.file_type();

                        // if this is a directory *and* reparse point then it's a junction,
                        // delete it like a directory but skip walking inside
                        if f_type.is_dir() && Cleaner::is_reparse_point(&metadata) {
                            fs::remove_dir(&path).unwrap_or_else(|e| {
                                error!("Failed to remove reparse point {}: {e}", path.display());
                            });
                            return None;
                        }

                        let mut perm = metadata.permissions();
                        if perm.readonly() {
                            #[allow(clippy::permissions_set_readonly_false)]
                            perm.set_readonly(false);
                            fs::set_permissions(&path, perm).unwrap_or_else(|e| {
                                error!("Error making {} write-accessible: {e}", path.display());
                            });
                        }
                        if f_type.is_file() {
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
