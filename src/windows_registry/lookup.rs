use std::io;

use winreg::RegKey;

use super::*;

impl Registry
{



    /// Will launch a recursive search through the registry. 
    /// .set_paths() and .set_root_XXXX() needs to be set prior to calling this function
    /// Registry Name: the "Name" of the value used by the registry (eg. UninstallString)
    /// Registry Data Regex: If the value under Registry Name matches the regex pattern 
    /// Depth: Depth of 0 looks at ONLY the key(s)/path(s), Depth of 1..x goes that many respective branches deeper.
    pub fn registry_search(&mut self, registry_name: &str, registry_data_regex: &str, depth: i32) -> Result<String, io::Error>
    {
        // Open a predefined registry key 
        let registry_key = RegKey::predef(self.registry_root);
        
        // setup the regex pattern information
        let regex_pattern = regex::Regex::new(registry_data_regex);
        if regex_pattern.is_err() { 
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Regex Pattern Invalid")); 
        }
        let regex = regex_pattern.unwrap();

        // Making the vector that will store all the paths to check data
        let mut all_paths: Vec<String> = Vec::new();

        // Check to make sure the paths open without error before saving them, reduce calculations/errors later
        for registry_path in &self.registry_paths
        {
            let current_path_wrapped = registry_key.open_subkey(registry_path);
            if current_path_wrapped.is_err()
            {
                continue;
            }
            all_paths.push(registry_path.to_string());
        }

        // Loop through the paths the amount of times that the user specified, which generates the depth
        #[allow(unused_labels)]
        'depth_loop: for _ in 0..depth
        {

            //iterates through every path in the vector
            'outer_path: for path in all_paths.clone()
            {
                // Opens the key to view the contents, jumps to the next iteration if it fails.
                // Will iterate through the next list of keys if it ok()
                let open_key_wrapped = registry_key.open_subkey(path.clone());
                if open_key_wrapped.is_err()
                {
                    continue 'outer_path;
                }
                let open_key = open_key_wrapped.unwrap();
                
                // Iterates through the list of keys inside the currently open path/key
                // jump to the next iteration if it fails, saves the full path to the vector if key is ok()
                'inner_path: for key in open_key.enum_keys()
                {
                    // makes sure the key isnt in-error before adding it to the vector of all paths
                    if key.is_err()
                    {
                        continue 'inner_path;
                    }

                    // saves the current path, appended with the new keys
                    // this will act as the next paths to search through if specified by the depth 
                    all_paths.push(path.clone() + "\\" + &key.unwrap());
                }
            }
        }

        // remove any duplicate entries, there shouldnt be duplicates but it is a preventative measure to prevent looking for values in the same directory multiple times
        all_paths.dedup();
        //println!("{:#?}", all_paths);

        for path in all_paths
        {
            let key = registry_key.open_subkey(path);
            if key.is_err()
            {
                continue;
            }

            let valid_key = key.unwrap();

            let registry_data_key: Result<String, io::Error> = valid_key.get_value(registry_name);
            if registry_data_key.is_err()
            {
                continue;
            }

            let registry_value = registry_data_key.unwrap();

            let Some(_caps) = regex.captures(&registry_value) else { continue; };

            return Ok(registry_value);

        }


        Err(io::Error::new(io::ErrorKind::NotFound, "Program Not Found"))
    }

}





impl Applications
{



    
    /// Searches the Registry using pre-determined values + the partial app name you provide to return a hashmap in a (appname,product id) pair
    /// # Examples 
    /// ```
    /// use davids_awesome_library::windows_registry::lookup::Registry;
    /// let mut product = Registry::new();
    /// product.find_msi_product_id("zip"); // searching for 7-zip
    /// ```
    pub fn find_msi_product_id(&mut self, app: &str) -> Result<std::collections::HashMap<String, String>, std::io::Error>
    {

        // The results will be saved here
        let mut app_results: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        
        // Sets the paths that we will use to check the registry. The root and path is predetermined in this function since we are gathering a known set of information
        let hklm = &self.reg_root;
        let reg_paths = &self.app_paths;

        // Check Both the paths
        for path in reg_paths
        {

            // Open the specific Registry Path
            let current_path = hklm.open_subkey(path)?;
            

            // Check the Keys
            for key in current_path.enum_keys()
            {

                // If the key is in error, jump to the next iteration (Key)
                if key.is_err()
                {
                    continue;
                }

                // We are now opening the key to view its contents
                let sub_key = current_path.open_subkey(key.unwrap())?;

                // Get the DisplayName and Uninstall String (Uninstall String contains the Product ID for MSI Installations)
                let disp_name: Result<String, std::io::Error> = sub_key.get_value("DisplayName");
                let uninst_string: Result<winreg::RegValue, std::io::Error> = sub_key.get_raw_value("UninstallString");
                

                // If there is an error, like an uninstall string not found, jump to the next iteration (Key)
                if disp_name.is_err() || uninst_string.is_err()
                {
                    continue;
                }
                
                // Formatting our data, some MSI uninstall strings are formatted with /I instead of /X,
                // Converting /I to /X and then checking for /X makes it easier to find matches later
                let app_name = disp_name.unwrap();
                let uninstall_string = uninst_string.unwrap().to_string().replace(" /I{", " /X{").replace("\\\"", "\"").replace("\"", "'");
                
                // exe uninstalls will not have /I{ or /X{, so if the UninstallString does not contain /X{, we move on to the next iteration
                if !uninstall_string.contains("/X{")
                {
                    continue;
                }

                // set everything to lowercase to ease matching
                if !app_name.to_ascii_lowercase().contains(&app.to_ascii_lowercase())
                {
                    continue;
                }

                // Currently the uninstall string looks like (for example) Msiexec.exe /X{3456345-55345-543543-5435345}, so we split it into 2 parts at /X, and keep the second half and return it.
                let product_id = uninstall_string.split("/X").last().unwrap().to_string();


                // Place the results into the hashmap, and continue on.
                app_results.insert(app_name, product_id);
            }
        }


        // Return the results if any exist, otherwise return an error.
        if app_results.is_empty()
        {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Program Not Found"))
        } else {
            
            return Ok(app_results);
        }
        
    }


    pub fn find_exe_uninstall_string(&mut self, app: &str) -> Result<std::collections::HashMap<String, Vec<String>>, std::io::Error>
    {

        // The results will be saved here
        let mut app_results: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        
        // Sets the paths that we will use to check the registry. The root and path is predetermined in this function since we are gathering a known set of information
        let hklm = &self.reg_root;
        let reg_paths = &self.app_paths;

        // Check Both the paths
        for path in reg_paths
        {

            // Open the specific Registry Path
            let current_path = hklm.open_subkey(path)?;
            
            // Check the Keys
            for key in current_path.enum_keys()
            {
                // If the key is in error, jump to the next iteration (Key)
                if key.is_err()
                {
                    continue;
                }

                // We are now opening the key to view its contents
                let sub_key = current_path.open_subkey(key.unwrap())?;

                // Get the DisplayName and Uninstall String
                let disp_name: Result<String, std::io::Error> = sub_key.get_value("DisplayName");
                let reg_string: Result<winreg::RegValue, std::io::Error> = sub_key.get_raw_value("UninstallString");
                
                // If there is an error, like an uninstall string not found, jump to the next iteration (Key)
                if disp_name.is_err() || reg_string.is_err()
                {
                    continue;
                }
                
                // Formatting our data, making it more functional later
                let app_name = disp_name.unwrap();
                let mut uninstall_string = reg_string.unwrap().to_string().replace("\\\"", "\"");
                
                
                // exe uninstalls will not start with MsiExec 
                if uninstall_string.starts_with("MsiExec.exe") || uninstall_string.is_empty()
                {
                    continue;
                }

                // removing the first quote because it is not needed and will only impede calling the uninstall
                uninstall_string.remove(0);
                
                // set everything to lowercase to ease matching
                if !app_name.to_ascii_lowercase().contains(&app.to_ascii_lowercase())
                {
                    continue;
                }


                

                // making a vector that we will use to return uninstall info
                let mut string_vec: Vec<String> = Vec::new();
                
                
                // The uninstall string ending in a quote means there are no parameters specified, the data can be formatted and returned to user
                if uninstall_string.ends_with("\"")
                {
                    
                    // push the value to the vector, removing the second quotation at the end since it is not needed
                    string_vec.push(uninstall_string.replace("\"", ""));
                    
                    // insert our data and continue on the loop
                    app_results.insert(app_name, string_vec);
                    continue;
                }


                // Split our vector in 2 parts, collect the data, save it to a Vec<String> from Vec<&str> and put the data in our hashmap
                // todo!("Split the second part of the vector into more parts and save it, for the apps that have multiple uninstall parameters");
                
                let uninstall_vec: Vec<&str> = uninstall_string.split("\" ").collect();
                for x in 0..uninstall_vec.len()
                {
                    string_vec.push(uninstall_vec[x].to_string());
                }
                

                // Place the results into the hashmap, and continue on.
                app_results.insert(app_name, string_vec);
            }
        }


        // Return the results if any exist, otherwise return an error.
        if app_results.is_empty()
        {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Program Not Found"))
        } else {
            
            return Ok(app_results);
        }
        
    }

    pub fn find_app_details(&mut self, app: &str) -> Result<std::collections::HashMap<String, String>, std::io::Error>
    {

        // The results will be saved here
        let mut app_results: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        
        // Sets the paths that we will use to check the registry. The root and path is predetermined in this function since we are gathering a known set of information
        let hklm = &self.reg_root;
        let reg_paths = &self.app_paths;

        // Check Both the paths
        for path in reg_paths
        {

            // Open the specific Registry Path
            let current_path = hklm.open_subkey(path)?;
            

            // Check the Keys
            for key in current_path.enum_keys()
            {

                // If the key is in error, jump to the next iteration (Key)
                if key.is_err()
                {
                    continue;
                }

                // We are now opening the key to view its contents
                let sub_key = current_path.open_subkey(key.unwrap())?;

                // Get the DisplayName and Uninstall String (Uninstall String contains the Product ID for MSI Installations)
                let disp_name: Result<String, std::io::Error> = sub_key.get_value("DisplayName");
                               

                // If there is an error, like an uninstall string not found, jump to the next iteration (Key)
                if disp_name.is_err()
                {
                    continue;
                }
                
                // Formatting our data, some MSI uninstall strings are formatted with /I instead of /X,
                // Converting /I to /X and then checking for /X makes it easier to find matches later
                let app_name = disp_name.unwrap();

                // set everything to lowercase to ease matching
                if !app_name.to_ascii_lowercase().contains(&app.to_ascii_lowercase())
                {
                    continue;
                }


                // get the list of information we may want
                let uninstall_string: Result<String, std::io::Error> = sub_key.get_value("UninstallString");
                let install_location: Result<String, std::io::Error> = sub_key.get_value("InstallLocation");
                let install_source: Result<String, std::io::Error> = sub_key.get_value("InstallSource");
                let display_icon: Result<String, std::io::Error> = sub_key.get_value("DisplayIcon");


                // Add it to our hashmap if ok and not in error
                if uninstall_string.is_ok()
                {
                    app_results.insert("UninstallString".to_string(), uninstall_string.unwrap());
                }

                if install_location.is_ok()
                {
                    app_results.insert("InstallLocation".to_string(), install_location.unwrap());
                }
                if install_source.is_ok()
                {
                    app_results.insert("InstallSource".to_string(), install_source.unwrap());
                }
                if display_icon.is_ok()
                {
                    app_results.insert("DisplayIcon".to_string(), display_icon.unwrap());
                }

                app_results.insert("DisplayName".to_string(), app_name);


                // return the results
                return Ok(app_results);
            }
        }


        // Return an error as no data was found.

        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Program Not Found"))

        
    }    

}