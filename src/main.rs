mod builtins;
mod shell;

fn main() {
    let lx: shell::Shell = Default::default();
    lx.run();
}
