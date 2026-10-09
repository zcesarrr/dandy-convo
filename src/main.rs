mod data;

struct Config {
    main_name: String,
    participant_name: String,
    variant: u8,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("not enough participants".to_string());
        }

        let main_name = args[1].clone();
        let participant_name = args[2].clone();
        let mut variant: u8 = 1;

        if args.len() >= 3 {
            for i in 3..args.len() {
                if args[i] == "--variant" {
                    let value = args.get(i + 1)
                        .ok_or_else(|| "missing variant number".to_string())?;

                    variant = value.parse()
                        .map_err(|_| {
                            "variant hasn't a valid number. Make sure to use a number between 1 and 255"
                        })?;

                    if variant == 0 { variant = 1 }
                }
            }
        }

        Ok(Self {
            main_name,
            participant_name,
            variant: variant - 1,
        })
    }
}

fn run(config: &Config) -> Result<(), String> {
    let build_dialogue = |main_name: &str, participant_name: &str|
        data::Dialogue::build(main_name, participant_name, config.variant);

    let main_dialogue = build_dialogue(
        &config.main_name, 
        &config.participant_name
    )?;
    let participant_dialogue = build_dialogue(
        &config.participant_name, 
        &config.main_name
    )?;

    if !main_dialogue.has_lines() {
        return Err(format!(
            "there's no interaction between {} and {} from variant {}", 
            main_dialogue.display_name(), 
            participant_dialogue.display_name(),
            config.variant + 1
        ));
    }

    data::Conversation::new(
        &main_dialogue, 
        &participant_dialogue
    )?.print();

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = run(&config) {
        eprintln!("Program error: {}", err);
        std::process::exit(1);
    }
}
