mod built_ins;

use built_ins::cd;
use std::{env, env::VarError, io, io::Write, path, process::Child};

const SHELL_NAME: &str = "lx";

type EnvVar = Result<String, VarError>;
type ReadLineResult = (bool, String);
type CommandArgs<'a> = std::str::SplitWhitespace<'a>;

pub struct Shell {
    prompt: String,
    style: ansi_term::Style,
    home_dir: Option<String>,
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            prompt: format!("{}>", SHELL_NAME),
            style: ansi_term::Style::new().bold().italic().underline(),
            home_dir: None,
        }
    }
}

impl Shell {
    pub fn run(&mut self) {
        self.home_dir = set_init_cwd(env::var("HOME"));

        loop {
            self.display_prompt();

            let (newline_entered, line) = read_line();

            if newline_entered || line.is_empty() {
                continue;
            }

            let mut args = line.split_whitespace();

            let cmd = args.next().unwrap();

            if self.launch_cmd_or_exit(cmd, args) {
                break;
            }
        }
    }

    fn display_prompt(&self) {
        print!("{} ", self.style.paint(self.prompt.clone()));
        io::stdout().flush().unwrap_or_default();
    }

    fn launch_cmd_or_exit(&self, cmd: &str, args: CommandArgs) -> bool {
        let mut exit = false;

        if cmd == "exit" {
            exit = true;
        } else if cmd == "cd" {
            self.change_directory(args.collect::<String>());
        } else {
            launch_process(cmd, args);
        }

        exit
    }

    fn change_directory(&self, path: String) {
        if let Err(e) = cd::cmd(
            &self.home_dir,
            if path.is_empty() { None } else { Some(path) },
        ) {
            error_msg("cd", &e.to_string());
        }
    }
}

fn set_init_cwd(home: EnvVar) -> Option<String> {
    let home_dir = home.map_or_else(|_| None, Some);

    if let Some(ref home_dir) = home_dir {
        env::set_current_dir(&path::Path::new(home_dir)).unwrap_or_else(|e| {
            warn_msg("Unable to set intial cwd", &e.to_string())
        })
    } else {
        warn_msg(
            "HOME environment variable not set",
            "unable to set initial cwd",
        )
    }

    home_dir
}

fn read_line() -> ReadLineResult {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap_or_default();

    (input == "\n", input.trim().to_string())
}

fn launch_process(command: &str, args: CommandArgs) {
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
            .paint(format!("{}: {}: {}", SHELL_NAME, prefix, message))
    )
}

fn warn_msg(prefix: &str, message: &str) {
    let warning_style = ansi_term::Style::new()
        .bold()
        .on(ansi_term::Color::Yellow)
        .fg(ansi_term::Color::Black);

    println!(
        "{}",
        warning_style.paint(format!("{}: {}: {}", SHELL_NAME, prefix, message))
    );
}
