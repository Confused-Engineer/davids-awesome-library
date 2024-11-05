use super::FolderSingle;
use super::FolderArray;

impl FolderSingle
{
    pub fn remove_folder(&mut self) -> std::io::Result<bool>
    {
        
        Ok(true)
    }

    pub fn remove_folder_and_contents(&mut self) -> std::io::Result<bool>
    {

        Ok(true)
    }
}

impl FolderArray {
    pub fn remove_folder(&mut self) -> std::io::Result<bool>
    {

        Ok(true)
    }

    pub fn remove_folder_and_contents(&mut self) -> std::io::Result<bool>
    {
        
        Ok(true)
    }  
}