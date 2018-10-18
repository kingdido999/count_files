use std::fs;
use std::path::Path;

pub fn count_files(dir: &Path) -> u32 {
    let mut count = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                count += 1;
            } else {
                count += count_files(&path);
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_files() {
        let path = Path::new("/bin");
        let count = count_files(path);
        assert!(count > 0);
    }
}
