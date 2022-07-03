use std::{io::Write, process::Child};

type ReadLineResult = (bool, String);

pub struct Shell {
    prompt: String,
    style: ansi_term::Style,
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            prompt: "lx>".to_string(),
            style: ansi_term::Style::new().bold().italic().underline(),
        }
    }
}

impl Shell {
    pub fn run(&self) {
        set_init_cwd();

        loop {
            self.display_prompt();

            let (newline_entered, line) = read_line();

            if newline_entered || line.is_empty() {
                continue;
            }

            if line == "exit" {
                break;
            }

            launch_cmd(line);
        }
    }

    fn display_prompt(&self) {
        print!("{} ", self.style.paint(self.prompt.clone()));
        std::io::stdout().flush().unwrap_or_default();
    }
}

fn set_init_cwd() {
    let home_dir = std::env::var("HOME").unwrap_or_default();

    if !home_dir.is_empty() {
        std::env::set_current_dir(&std::path::Path::new(&home_dir))
            .unwrap_or_else(|e| {
                warn_msg("Unable to set intial cwd", &e.to_string())
            });
    } else {
        warn_msg(
            "HOME environment variable not set",
            "unable to set initial cwd",
        );
    }
}

fn read_line() -> ReadLineResult {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap_or_default();

    (input == "\n", input.trim().to_string())
}

fn launch_cmd(input: String) {
    let mut args = input.split_whitespace();

    let command = args.next().unwrap();

    launch(command, args);
}

fn launch(command: &str, args: std::str::SplitWhitespace) {
    let on_wait = |mut c: Child| {
        let status = c.wait();

        match status {
            Ok(_) => (),
            Err(e) => error_msg("I/O error", &e.to_string()),
        }
    };

    match std::process::Command::new(command).args(args).spawn() {
        Ok(child) => {
            on_wait(child);
        }
        Err(_) => error_msg("command not found", command),
    }
}

fn error_msg(prefix: &str, message: &str) {
    eprintln!(
        "{}",
        ansi_term::Color::Red
            .bold()
            .paint(format!("{}: {}: {}", "lx", prefix, message))
    )
}

fn warn_msg(prefix: &str, message: &str) {
    let warning_style = ansi_term::Style::new()
        .bold()
        .on(ansi_term::Color::Yellow)
        .fg(ansi_term::Color::Black);

    eprintln!(
        "{}",
        warning_style.paint(format!("{}: {}: {}", "lx", prefix, message))
    );
}
