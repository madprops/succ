// The program version
const VERSION: &str = "v1.3.0";

mod macros;
mod args;
mod files;
mod input;

use crate::
{
    args::
    {
        check_args
    },
    files::
    {
        is_file,
        get_parent,
        get_file_names,
        check_path, cwd
    },
    input::
    {
        ask_bool
    }
};

use std::
{
    process, fs,
    path::PathBuf
};

// Program starts here
fn main()
{
    // Check for arguments
    let args = check_args();

    // Check if path is valid
    let path = match args.0
    {
        Ok(buff) => buff,
        Err(_) => exit("Invalid path.")
    };

    // Additonal checks to
    // make sure the operations
    // can be performed
    match check_path(&path)
    {
        Ok(_) => {},
        Err(e) => exit(&e)
    }

    // Get other arguments ready
    let print = !args.1;
    let replace = !args.3;
    let delete = !args.4;

    // If --yes is not provided 
    // then show the confirmation prompt
    // Exit on a non-positive response
    if !args.2
    {
        // Show the source directory path
        p!("This will succ {}", path.to_str().unwrap());

        if cwd() == path
        {
            // Show that this is the current working directory
            // as one way to avoid accidental operations
            p!("That is the current working directory.")
        }

        if delete
        {
            // If --no-delete wasn't provided then notify the user 
            // that the directory will be deleted
            p!("It will delete the directory after moving the files.");
        }

        if !ask_bool("Proceed?", true)
        {
            // Not a confirmation
            // Exit the program
            exit("");
        }
    }

    // Perform the file operations
    succ(path, print, replace, delete);
}

// Custom function to exit
// with an optional message
fn exit(s: &str) -> !
{
    if !s.is_empty()
    {
        p!(s);
    }

    process::exit(0)
}

// Moves the files to the parent directory
// Some actions are enabled or disabled by arguments
fn succ(path: PathBuf, print: bool, replace: bool, delete: bool)
{
    // Try to get the parent path
    match get_parent(&path)
    {
        Ok(parent_buff) =>
        {
            // Rename directory to something else
            // This is to avoid collisions

            let sname = format!("__succ__{}", path.file_name().unwrap()
                                                    .to_str().unwrap());
            let spath = parent_buff.join(sname);

            match fs::rename(&path, &spath)
            {
                Ok(_) => {},
                Err(e) => exit(&s!(e))
            }

            // Get the directory files
            match get_file_names(&spath)
            {
                Ok(buffs) =>
                {
                    // Get parent files
                    let parent_names = match get_file_names(&parent_buff)
                    {
                        Ok(buffs) =>
                        {
                            // Get the file names for comparisons
                            buffs.iter().map(|b| b.file_name().unwrap()
                                        .to_str().unwrap())
                                        .map(|s| s!(s))
                                        .collect::<Vec<String>>()
                        },
                        Err(e) => exit(&e)
                    };

                    // Start move

                    for file in buffs.iter()
                    {
                        // Get the source file name
                        let name = file.file_name().unwrap()
                                        .to_str().unwrap();
                        
                        // Get the target file path
                        let npath = parent_buff.join(name);

                        // If a file or dir with the same 
                        // name exists, remove it
                        if parent_names.contains(&s!(name))
                        {
                            // If no-replace arg was supplied
                            // leave the original file intact
                            if !replace {continue}

                            // Check if original is a file
                            // or a directory to know what
                            // delete method to use
                            if is_file(&npath.to_path_buf())
                            {
                                // Remove file
                                match fs::remove_file(&npath)
                                {
                                    Ok(_) => {},
                                    Err(e) => exit(&s!(e))
                                }
                            }

                            else
                            {
                                // Remove directory
                                match fs::remove_dir_all(&npath)
                                {
                                    Ok(_) => {},
                                    Err(e) => exit(&s!(e))
                                }
                            }
                        }

                        // Move file to parent
                        match fs::rename(&file, &npath)
                        {
                            Ok(_) => if print {p!("Moved: {}", name)},
                            Err(e) => exit(&s!(e))
                        }
                    }

                    if delete
                    {
                        // If delete arg is true
                        // remove empty directory
                        match fs::remove_dir_all(&spath)
                        {
                            Ok(_) => if print {p!("Directory deleted.")},
                            Err(e) => exit(&s!(e))
                        }
                    }

                    // Everything is done
                    if print {p!("Done!")}
                },
                Err(e) => exit(&e)
            }
        },
        Err(e) => exit(&e)
    }
}