use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let app = App::new("multiple_app")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(|c: &Context| {
            println!("{:?} : {}", c.args, c.bool_flag("bool"));
            Ok(())
        })
        .flag(
            Flag::new("bool", FlagType::Bool)
                .description("bool flag")
                .alias("b"),
        )
        .command(add_command())
        .command(hello_command());

    return app.run(args);
}

fn hello_action(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }

    match c.int_flag("age") {
        Ok(age) => println!("{:?} is {} years old", c.args, age),
        Err(e) => eprintln!("{}", e),
    }

    match c.string_flag("neko") {
        Ok(neko) => println!("neko say {}", neko),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}

fn hello_command() -> Command {
    Command::new("hello")
        .description("hello command")
        .usage("multiple_app hello(he, h) [name]")
        .alias("h")
        .alias("he")
        .action(hello_action)
        .flag(
            Flag::new("bye", FlagType::Bool)
                .description("bye flag")
                .alias("b"),
        )
        .flag(
            Flag::new("age", FlagType::Int)
                .description("age flag")
                .alias("a")
                .alias("ag"),
        )
}

fn add_action(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum();
    println!("{}", sum);
    Ok(())
}

fn add_command() -> Command {
    Command::new("add")
        .description("add command")
        .usage("multiple_app add [num...]")
        .action(add_action)
}
