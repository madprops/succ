use crate::
{
    s
};

use std::
{
    fs, env,
    path::PathBuf
};

// Gets the current working directory
pub fn cwd() -> String
{
    env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// Gets the path parent
pub fn get_parent(path: &PathBuf) -> Result<PathBuf, String>
{
    match path.parent()
    {
        Some(pth) =>
        {
            Ok(pth.to_path_buf())
        },
        None => Err(s!("Can't get the parent directory."))
    }
}

// Gets the absolute path
pub fn absolute_path(path: &str) -> Result<PathBuf, std::io::Error>
{
    fs::canonicalize(path)
}

// Gets the list of filenames in a path
pub fn get_file_names(path: &PathBuf) -> Result<Vec<PathBuf>, String>
{
    let mut names: Vec<PathBuf> = vec![];

    match fs::read_dir(path)
    {
        Ok(files) =>
        {
            for file in files
            {
                names.push(file.unwrap().path());
            }
        },
        Err(_) =>
        {
            return Err(s!("Can't read directory."));
        }
    }

    Ok(names)
}

// Returns the number of files
pub fn count_files(path: &PathBuf) -> usize
{
    match fs::read_dir(path)
    {
        Ok(files) =>
        {
            files.count()
        },
        Err(_) =>
        {
            0
        }
    }
}

// True if file 
// False if directory
pub fn is_file(path: &PathBuf) -> bool
{
    fs::metadata(path).unwrap().is_file()
}

// Checks if the path is valid for succ
pub fn check_path(path: &PathBuf) -> Result<bool, String>
{
    if is_file(path)
    {
        return Err(s!("This is not a directoy path."));
    }

    if count_files(path) == 0
    {
        return Err(s!("This is an empty directory."));
    }

    Ok(true)
}