mod macros;
mod args;
mod files;

use crate::
{
    args::
    {
        check_args
    },
    files::
    {
        get_parent,
        get_file_names,
        is_file
    }
};

use std::
{
    process, fs,
    path::{Path, PathBuf},
    io::{self, Write}
};

// Program starts here
fn main()
{
    let args = check_args();

    let path = match args.0
    {
        Ok(buff) => buff,
        Err(_) => exit("Invalid path.")
    };

    let print = !args.1;

    if !args.2
    {
        let ans = &ask_input("Proceed? (y/n)")
                    .trim().to_lowercase()[..];

        if ans != "y"
        {
            exit("");
        }
    }

    succ(path, print);
}

fn exit(s: &str) -> !
{
    if !s.is_empty()
    {
        p!(s);
    }

    process::exit(0)
}

fn ask_input(s: &str) -> String
{
    pp!(format!("{}: ", s));
    io::stdout().flush().unwrap();

    let mut input = s!();

    match io::stdin().read_line(&mut input) 
    {
        Ok(_) => input,
        Err(_) => s!()
    }
}

fn succ(path: PathBuf, print: bool)
{
    match get_parent(&path)
    {
        Ok(buff) =>
        {
            match get_file_names(&path)
            {
                Ok(buffs) =>
                {
                    if buffs.is_empty()
                    {
                        exit("Directory is empty.");
                    }

                    // Get parent files

                    let parent = s!(buff.to_str().unwrap());
            
                    let parent_names = match get_file_names(&buff)
                    {
                        Ok(buffs) =>
                        {
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
                        let name = file.file_name().unwrap()
                                        .to_str().unwrap();
                        
                        let fname = format!("{}/{}", parent, name);
                        let npath = Path::new(&fname);

                        // If a file or dir with the same 
                        // name exists, remove it
                        if parent_names.contains(&s!(name))
                        {
                            if is_file(&npath.to_path_buf())
                            {
                                match fs::remove_file(npath)
                                {
                                    Ok(_) => {},
                                    Err(e) => exit(&s!(e))
                                }
                            }

                            else
                            {
                                match fs::remove_dir_all(npath)
                                {
                                    Ok(_) => {},
                                    Err(e) => exit(&s!(e))
                                }
                            }
                        }

                        // Move file to parent
                        match fs::rename(file, npath)
                        {
                            Ok(_) => if print {p!("Copied: {}", name)},
                            Err(e) => exit(&s!(e))
                        }
                    }

                    // Remove empty file
                    match fs::remove_dir_all(&path)
                    {
                        Ok(_) => if print {p!("Done!")},
                        Err(e) => exit(&s!(e))
                    }
                },
                Err(e) =>
                {
                    exit(&e)
                }
            }
        },
        Err(e) => exit(&e)
    }
}