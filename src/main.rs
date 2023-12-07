use std::fs;

fn main() {
    let dir = std::fs::read_dir(
        r#"C:\Users\thega\AppData\Roaming\yuzu\nand\user\save\0000000000000000\F0748197393EEDCDE71C4EE207801D64\0100F2C0115B6000\"#,
    )
    .unwrap();

    let new_dir = r#"C:\Users\thega\Documents\totk_save_backups\sav_"#.to_owned()
        + &rand::random::<u32>().to_string();

    let mut dir_builder = std::fs::DirBuilder::new();

    dir_builder.recursive(true);
    dir_builder.create(&new_dir).expect("");
    dir_builder.create(new_dir.clone() + "/picturebook/").expect("");

    for file in dir {
        let file = file.unwrap().path();

        if file.is_dir() {
            match file.file_name().unwrap().to_str().unwrap() {
                "slot_00" => {
                    for item in file.read_dir().unwrap() {
                        let item = item.unwrap();

                        fs::copy(
                            item.path(),
                            new_dir.to_owned() + "/" + item.file_name().to_str().unwrap(),
                        )
                        .expect("couldnt copy file");
                    }
                }

                "picturebook" => {
                    for item in file.read_dir().unwrap() {
                        let item = item.unwrap();

                        fs::copy(
                            item.path(),
                            new_dir.to_owned() + "/picturebook/" + item.file_name().to_str().unwrap(),
                        )
                        .expect("couldnt copy file");
                    }
                }

                _ => continue,
            }
        }
    }
}
