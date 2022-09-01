use std::{
    fs::{self, DirEntry},
    io::Result,
    path::Path,
};

pub fn visit_files(dir: &Path, cb: &dyn Fn(&DirEntry)) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_files(&path, cb)?;
        } else {
            cb(&entry);
        }
    }
    Ok(())
}
