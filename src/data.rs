pub struct Dialogue {
    display_name: String,
    lines: Vec<String>,
}

impl Dialogue {
    pub fn build(name: &str, participant_name: &str, target_variant: u8) -> Result<Self, String> {
        let contents = std::fs::read_to_string(
            format!("dialogues/{}", name)
        ).map_err(|_| format!("\"{}\" is not a valid name", name))?;

        let contents_lines: Vec<&str> = contents.lines().collect();
        let contents_lines_length = contents_lines.len();

        let mut variant_index = 0;
        let mut dialogue_lines: Vec<String> = Vec::new();
        'main: for i in 0..contents_lines_length {
            if contents_lines[i] != format!(".{}", participant_name) {
                continue;
            }

            if variant_index < target_variant {
                variant_index += 1;
                continue;
            }

            for j in i+1..contents_lines_length {
                if j == contents_lines_length {
                    break 'main;
                }

                if contents_lines[j] != "" {
                    dialogue_lines.push(contents_lines[j].to_string());
                    continue;
                }

                break 'main;
            }
        }

        Ok(Self {
            display_name: contents_lines[0].to_string(),
            lines: dialogue_lines,
        })
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn has_lines(&self) -> bool {
        self.lines.len() > 0
    }
}

pub struct Conversation {
    lines: Vec<String>,
}

impl Conversation {
    pub fn new(main_dialogue: &Dialogue, participant_dialogue: &Dialogue) -> Result<Self, String> {
        let main_dialogue_length = main_dialogue.lines.len();
        if main_dialogue_length != participant_dialogue.lines.len() {
            return Err(format!(
                "{}. {}",
                "the two dialogues aren't the same length (invalid dialogue)",
                "This may be an error by the developer, or you may have unintentionally modified the dialogue lines"
            ));
        }
        
        let mut lines: Vec<String> = Vec::new();
        let mut push_format_line = |display_name: &str, text: &str| {
            lines.push(format!("{}: {}", display_name, text));
        };

        for i in 0..main_dialogue_length {
            let main_line = &main_dialogue.lines[i];

            if main_line != "'" {
                push_format_line(&main_dialogue.display_name, &main_line);
            } else {
                push_format_line(&participant_dialogue.display_name, &participant_dialogue.lines[i]);
            }
        }
        
        Ok(Self {
            lines,
        })
    }

    pub fn print(&self) {
        for line in &self.lines {
            println!("{}", line);
        }
    }
}
