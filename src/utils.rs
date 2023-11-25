use std::path::PathBuf;

pub fn expand_home_dir(path: impl AsRef<str>) -> PathBuf {
    let path = path.as_ref().replace(
        '~',
        &std::env::var("HOME").expect("$HOME env var not found."),
    );
    PathBuf::from(path)
}
