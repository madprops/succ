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
        check_path
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
    let args = check_args();

    let path = match args.0
    {
        Ok(buff) => buff,
        Err(_) => exit("Invalid path.")
    };

    match check_path(&path)
    {
        Ok(_) => {},
        Err(e) => exit(&e)
    }

    let print = !args.1;

    if !args.2
    {
        p!("This will succ {}", path.to_str().unwrap());

        if !ask_bool("Proceeed?", true)
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

fn succ(path: PathBuf, print: bool)
{
    match get_parent(&path)
    {
        Ok(parent_buff) =>
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
            
                    let parent_names = match get_file_names(&parent_buff)
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
                        // Get the source file name
                        let name = file.file_name().unwrap()
                                        .to_str().unwrap();
                        
                        // Get the target file path
                        let npath = parent_buff.join(name);

                        // If a file or dir with the same 
                        // name exists, remove it
                        if parent_names.contains(&s!(name))
                        {
                            if is_file(&npath.to_path_buf())
                            {
                                match fs::remove_file(&npath)
                                {
                                    Ok(_) => {},
                                    Err(e) => exit(&s!(e))
                                }
                            }

                            else
                            {
                                match fs::remove_dir_all(&npath)
                                {
                                    Ok(_) => {},
                                    Err(e) => exit(&s!(e))
                                }
                            }
                        }

                        // Move file to parent
                        match fs::rename(file, &npath)
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