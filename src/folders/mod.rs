mod remove;
mod read;
mod create;
mod modify;

/// Used to create/delete/modify/read folders and its contents
pub struct FolderSingle
{
    dir: String
}

impl FolderSingle
{
    pub fn new() -> Self
    {
        FolderSingle { dir: String::new() }
    }

    pub fn set_root_directory(&mut self, dir: &str) -> &mut Self
    {
        self.dir = dir.to_string();
        self
    }
}

pub struct FolderArray
{
    dirs: Vec<String>
}

impl FolderArray
{
    pub fn new() -> Self
    {
        FolderArray { dirs: Vec::new() }
    }

    pub fn set_root_directory(&mut self, dirs: Vec<String>) -> &mut Self
    {
        self.dirs = dirs;
        self
    }
}


