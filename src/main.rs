use anyhow::Context as _;

use rucco::core;
use rucco::types;

fn repl() -> anyhow::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix("rucco").context("Failed to get XDG directories")?;
    let history_file = xdg_dirs
        .place_config_file("history.txt")
        .context("Failed to get history file path")?;
    let history_file_path = history_file.as_path();
    _ = rl.load_history(history_file_path);

    // let mut env = core::default_env();
    let mut env = std::collections::HashMap::new();

    loop {
        let line = rl.readline("rucco> ");
        match &line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let res = core::rep(line, &mut env);

                match res {
                    Ok(res) => println!("{}", res),
                    Err(e) => {
                        if let Some(types::RuccoReplErr::EmptyInput) = e.downcast_ref() {
                            break;
                        };
                        eprintln!("{:#?}", e);
                    }
                }
            }
            Err(
                rustyline::error::ReadlineError::Interrupted | rustyline::error::ReadlineError::Eof,
            ) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
    }
    rl.save_history(history_file_path).with_context(|| {
        format!(
            "Failed save history file: {}",
            history_file_path.to_str().unwrap()
        )
    })?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    repl()
}
