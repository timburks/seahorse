use seahorse::{App, Context, Flag, FlagType};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let name = "single_app";

    let app = App::new(name)
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("single_app [args]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flag(
            Flag::new("bye", FlagType::Bool)
                .description("single_app args --bye(-b)")
                .alias("b"),
        );

    return app.run(args);
}

fn action(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }
    Ok(())
}
