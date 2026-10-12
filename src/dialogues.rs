const DIALOGUES_DIR: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/dialogues");

pub fn get_contents(toon_name: &str) -> Result<String, String> {
    let file = DIALOGUES_DIR
        .get_file(toon_name)
        .ok_or_else(|| format!("\"{}\" is not a valid name", toon_name))?;

    let contents = file.contents_utf8()
        .ok_or_else(|| format!("the file \"{}\" is not valid UTF-8", toon_name))?;

    Ok(contents.to_owned())
}

pub fn get_names() -> impl Iterator<Item = (String, String)> {
    DIALOGUES_DIR
        .files()
        .map(|file| {
            let contents = file.contents_utf8().unwrap();
            (
                file.path().file_name().unwrap().to_string_lossy().into_owned(),
                contents.lines().next().unwrap().to_owned(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_dialogues_contents() {
        let gigi_dialogues = get_contents("gigi").unwrap();

        assert!(gigi_dialogues.contains("Heeey wassup?"));
    }

    #[test]
    fn getting_dialogue_names() {
        let mut names = get_names();

        assert!(names.any(|(id, _)| id == "gigi"));
        assert!(names.any(|(_, name)| name == "Razzle & Dazzle"));
    }
}
