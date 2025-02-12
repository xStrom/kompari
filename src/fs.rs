use std::ffi::OsString;
use std::path::Path;

pub fn read_images_from_dir(path: &Path) -> crate::Result<Vec<OsString>> {
    Ok(std::fs::read_dir(path)?
        .flatten()
        .filter_map(|path| {
            if path
                .path()
                .extension()
                .and_then(|p| p.to_str())
                .map_or(false, |p| p.to_ascii_lowercase() == "png")
            {
                Some(path.file_name())
            } else {
                None
            }
        })
        .collect())
}
