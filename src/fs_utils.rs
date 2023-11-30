use std::path::PathBuf;

pub fn recursive_copy(file: PathBuf, new_dir: &String) {
    for item in file.read_dir().unwrap() {
        let item = item.unwrap();

        std::fs::copy(
            item.path(),
            new_dir.to_owned() + "/slot_00/" + item.file_name().to_str().unwrap(),
        )
        .expect("couldnt copy file");
    }
}
