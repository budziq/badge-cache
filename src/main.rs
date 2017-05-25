extern crate badge_cache;
#[macro_use] extern crate clap;

use badge_cache::service;
use badge_cache::errors::*;

use clap::{Arg, App, SubCommand, ArgMatches};

pub fn main() {
    let matches = App::new("badge-cache")
        .version(crate_version!())
        .about("Shields IO Badge Caching Server")
        .subcommand(SubCommand::with_name("serve")
                    .about("Initialize Server")
                    .arg(Arg::with_name("port")
                         .long("port")
                         .short("p")
                         .takes_value(true)
                         .help("Port to listen on. Defaults to 3000"))
                    .arg(Arg::with_name("public")
                         .long("public")
                         .help("Serve on '0.0.0.0' instead of 'localhost'"))
                    .arg(Arg::with_name("silent")
                         .long("silent")
                         .help("Don't output any logging info")))
        .subcommand(SubCommand::with_name("admin")
                    .about("admin functions")
                    .arg(Arg::with_name("refresh")
                         .long("refresh")
                         .takes_value(false)
                         .help("Refresh cached badges"))
                    .arg(Arg::with_name("no-confirm")
                         .long("no-confirm")
                         .takes_value(false)
                         .help("Auto-confirm/skip any confirmation checks")))
        .get_matches();

    if let Err(error) = run(matches) {
        println!("Error: {}", error);
        ::std::process::exit(1);
    }
}


fn run(matches: ArgMatches) -> Result<()> {
    if let Some(serve_matches) = matches.subcommand_matches("serve") {
        let port = serve_matches.value_of("port").unwrap_or("3000");
        let host_base = if serve_matches.is_present("public") { "0.0.0.0" } else { "localhost" };
        let host = format!("{}:{}", host_base, port);
        let do_log = !serve_matches.is_present("silent");
        let db_url = serve_matches.value_of("database");
        service::start(&host, db_url, do_log);
        return Ok(());
    }

    if let Some(_admin_matches) = matches.subcommand_matches("admin") {
        //return admin::handle(admin_matches)
        return Ok(());
    }

    println!("badge-cache: see `--help`");
    Ok(())
}


