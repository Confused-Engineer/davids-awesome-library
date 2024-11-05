use super::*;

impl Registry
{
    /// # Description
    /// Remove Keys from the Windows Registry.
    /// These are the 'Folders' and not the data entries within the keys.
    /// # Examples
    /// ```
    /// let mut reg = davids_awesome_library::windows_registry::Registry::new();
    /// reg.set_root_hklm();
    /// ```
    pub fn remove_keys(&mut self, key_vec: Vec<String>) -> std::io::Result<()>
    {
        
        if key_vec.is_empty()
        {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        // Open a predefined registry key 
        let registry_key = winreg::RegKey::predef(self.registry_root);
        
        // Making the vector that will store all the paths to check data
        let mut all_paths_untested: Vec<String> = Vec::new();

        let mut all_paths: Vec<String> = Vec::new();

        // Check to make sure the paths open without error before saving them, reduce calculations/errors later
        for registry_path in &self.registry_paths
        {
            for key in key_vec.iter()
            {
                all_paths_untested.push(format!("{}\\{}", registry_path, key));
            }
            
        }

        all_paths_untested.dedup();

        for path in all_paths_untested
        {
            let reg_test = registry_key.open_subkey(path.clone());
            if reg_test.is_err()
            {
                continue;
            }

            all_paths.push(path.to_string());
        }

        if all_paths.is_empty()
        {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        for path in all_paths
        {
            let _ = registry_key.delete_subkey_all(path);
            
        }



        Ok(())
        
    
    }
}
