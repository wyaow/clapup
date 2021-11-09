use clap::{Arg, App};

fn main() {
    let matches = App::new("A toolkit used to diagnosis services")
        .version("1.0")
        .author("allen <94291@sangfor.com>")
        .about("Let's together make IT simple more simple")
        .arg(Arg::new("service")
            .about("Sets the service to use")
            .value_name("ServiceName")
            .required(true)
            .index(1))
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("ConfigFile")
            .about("Sets config file of your service")
            .takes_value(true))
        .arg(Arg::new("log")
            .short('l')
            .long("log")
            .value_name("LogPath")
            .about("Sets log path of your service")
            .takes_value(true))
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .takes_value(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("test")            // subcommand
            .about("controls testing features")
            .version("1.0.0")
            .author("allen <94291@sangfor.com>")
            .arg(Arg::new("debug")
                .about("debug info detail")
                .short('d')
                .long("debug")
                .about("print debug information verbosely")))
        .subcommand(App::new("check")
            // .short_flag('c')
            .long_flag("check")
            .about("check whether service is online and healthy")
            .version("1.0.0")
            .author("allen <94291@sangfor.com>")
            .arg(Arg::new("net")
                .about("check network status[net,log,ping]")
                .value_name("category")
                .required(true)
                .index(1)))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(i) = matches.value_of("INPUT") {
        println!("Value for input: {}", i);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off 0"),
        1 => println!("Verbose mode is kind of on 1"),
        2 => println!("Verbose mode is on 2"),
        _ => println!("Don't be crazy 3+"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("debug") {
            // "$ myapp test -d" was run
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // Continued program logic goes here...
}