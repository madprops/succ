use crate::
{
    s,
    files::
    {
        cwd,
        absolute_path
    }
};

use std::path::PathBuf;
use clap::{App, Arg};

// Starts the argument system
pub fn check_args() -> (Result<PathBuf, std::io::Error>, bool, bool, bool, bool)
{
    let matches = App::new("succ")
    .version("v1.2.2")
    .about("Moves all contents of a dir to the parent dir and removes the empty dir")
    .arg(Arg::with_name("PATH")
        .help("Use a custom file path")
        .required(false)
        .index(1))
    .arg(Arg::with_name("path")
        .long("path")
        .value_name("Path")
        .help("Use a custom file path")
        .takes_value(true))
    .arg(Arg::with_name("silent")
        .long("silent")
        .multiple(false)
        .help("Show no output except errors"))
    .arg(Arg::with_name("yes")
        .long("yes")
        .multiple(false)
        .help("Confirm the operation automatically"))
    .arg(Arg::with_name("no-replace")
        .long("no-replace")
        .multiple(false)
        .help("Don't replace existing files/dirs with the same name"))
    .arg(Arg::with_name("no-delete")
        .long("no-delete")
        .multiple(false)
        .help("Don't delete the directory after moving files"))
    .get_matches();
    
    let path;

    // Check for normal path argument
    if let Some(pth) = matches.value_of("PATH")
    {
        path = s!(pth);
    }

    // If not check for option path argument
    else if let Some(pth) = matches.value_of("path")
    {
        path = s!(pth);
    }

    else
    {
        // Else use default path
        path = cwd();
    }

    let silent = matches.occurrences_of("silent") > 0;
    let confirm = matches.occurrences_of("yes") > 0;
    let no_replace = matches.occurrences_of("no-replace") > 0;
    let no_delete = matches.occurrences_of("no-delete") > 0;

    (absolute_path(&path), silent, confirm, no_replace, no_delete)
}