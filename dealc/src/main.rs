mod phrase;

use std::{fs, path::PathBuf};

use shared::vision::Annotation;

fn main() {
    for file in fs::read_dir("dealc/.local").unwrap() {
        let file = file.unwrap();

        if file.path().extension().is_some() {
            continue;
        }

        let json: Annotation =
            serde_json::from_str(&fs::read_to_string(file.path()).unwrap())
                .unwrap();

        let phrases = phrase::from_text(&json.text).join("\n");
        fs::write(
            PathBuf::from("clis/phrases")
                .join(file.path().file_name().unwrap()),
            phrases,
        )
        .unwrap();
    }
}
