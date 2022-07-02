use ansi_term;
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
    pub fn run(&mut self) {
        Shell::set_init_cwd();

        loop {
            self.display_prompt();

            let (newline_entered, line) = Shell::read_line();

            if newline_entered || line.is_empty() {
                continue;
            }

            if line == "exit" {
                break;
            }

            self.launch_cmd(line);
        }
    }

    fn set_init_cwd() {
        let home_dir = std::env::var("HOME").unwrap_or_default();

        if !home_dir.is_empty() {
            std::env::set_current_dir(&std::path::Path::new(&home_dir)).unwrap_or_default();
        } else {
            Shell::error_msg(
                "HOME environment variable not set",
                "unable to set initial cwd",
            );
        }
    }

    fn display_prompt(&self) {
        print!("{} ", self.style.paint(self.prompt.clone()));
        std::io::stdout().flush().unwrap_or_default();
    }

    fn read_line() -> ReadLineResult {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap_or_default();

        (input == "\n", input.trim().to_string())
    }

    fn launch_cmd(&mut self, input: String) {
        let mut args = input.split_whitespace();

        let command = args.next().unwrap();

        Shell::launch(command, args);
    }

    fn launch(command: &str, args: std::str::SplitWhitespace) {
        let on_wait = |mut c: Child| {
            let status = c.wait();

            match status {
                Ok(_) => (),
                Err(e) => Shell::error_msg("I/O error", &e.to_string()),
            }
        };

        match std::process::Command::new(command).args(args).spawn() {
            Ok(child) => {
                on_wait(child);
            }
            Err(_) => Shell::error_msg("command not found", command),
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
}
