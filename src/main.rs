use fs_utils::recursive_copy;

mod fs_utils;

fn main() {
    let dir = std::fs::read_dir(
        r#"C:\Users\thega\AppData\Roaming\Ryujinx\bis\user\save\0000000000000001\0"#,
    )
    .unwrap();

    let new_dir = r#"C:\Users\thega\Documents\totk_save_backups\sav_"#.to_owned()
        + &rand::random::<u128>().to_string();

    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&new_dir)
        .expect("");

    for file in dir {
        let file = file.unwrap().path();

        if file.is_dir() {
            match file.file_name().unwrap().to_str().unwrap() {
                "slot_00" => {
                    recursive_copy(file, &new_dir);
                }

                "picturebook" => {
                    recursive_copy(file, &new_dir);
                }

                _ => continue,
            }
        }
    }
}
