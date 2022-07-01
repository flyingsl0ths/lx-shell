use ansi_term;
use std::io::Write;

type ReadLineResult = (bool, String);

const DEFAULT_PROMPT: &str = "lx>";
const SHELL_NAME: &str = "lx";
const CMD_NOT_FOUND_PREFIX: &str = "command not found";

pub struct Shell {
    prompt: String,
    style: ansi_term::Style,
    quit: bool,
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            prompt: DEFAULT_PROMPT.to_string(),
            style: ansi_term::Style::new().bold().italic().underline(),
            quit: false,
        }
    }
}

impl Shell {
    pub fn run(&mut self) {
        loop {
            self.display_prompt();

            let (no_command_given, line) = Shell::read_line();

            if no_command_given {
                continue;
            }

            self.launch_cmd(line);

            if self.quit {
                break;
            }
        }
    }

    fn display_prompt(&self) {
        print!("{} ", self.style.paint(self.prompt.clone()));
        std::io::stdout().flush().unwrap();
    }

    fn read_line() -> ReadLineResult {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap_or_default();

        (input == "\n", input)
    }

    fn launch_cmd(&mut self, input: String) {
        let mut args = input.trim().split_whitespace();

        let command = args.next().unwrap();

        if Shell::exit_or_launch(command, args) {
            self.quit = true;
            return;
        }
    }

    fn exit_or_launch(command: &str, args: std::str::SplitWhitespace) -> bool {
        if command == "exit" {
            return true;
        }

        Shell::launch(command, args);
        false
    }

    fn launch(command: &str, args: std::str::SplitWhitespace) {
        match std::process::Command::new(command).args(args).spawn() {
            Ok(mut child) => {
                child.wait().unwrap();
                ()
            }
            Err(_) => eprintln!("{}", Shell::error_msg(CMD_NOT_FOUND_PREFIX, command)),
        }
    }

    fn error_msg<'a>(
        prefix: &'static str,
        command: &'a str,
    ) -> ansi_term::ANSIGenericString<'a, str> {
        ansi_term::Color::Red
            .bold()
            .paint(format!("{}: {}: {}", SHELL_NAME, prefix, command))
    }
}
