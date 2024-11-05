/// A series of funtions to lookup data inside of the windows registry
mod lookup;
mod remove;

use winreg::enums::*;
/// Used for setting Registry details for looking up information 
pub struct Registry
{
    registry_root: isize,
    registry_paths: Vec<String>,

}

impl Default for Registry
{
    fn default() -> Self {
        Self {
            registry_root: 0,
            registry_paths: vec![String::new()],
        }
    }

    
}

impl Registry
{
    
    /// Sets default values
    /// These salues do not work by default but will suffice for some functions like find_msi_product_id which use preset values
    pub fn new() -> Self
    {
        Self::default()
    }

    /// set to "HKEY_LOCAL_MACHINE" for searching the registry
    pub fn set_root_hklm(&mut self) -> &mut Self
    {
        self.registry_root = HKEY_LOCAL_MACHINE;
        self
    }

    /// set to "HKEY_CLASSES_ROOT" for searching the registry
    pub fn set_root_hkcr(&mut self) -> &mut Self
    {
        self.registry_root = HKEY_CLASSES_ROOT;
        self
    }
    
    /// set to "HKEY_CURRENT_USER" for searching the registry
    pub fn set_root_hkcu(&mut self) -> &mut Self
    {
        self.registry_root = HKEY_CURRENT_USER;
        self
    }
    
    /// set to "HKEY_USERS" for searching the registry
    pub fn set_hku(&mut self) -> &mut Self
    {
        self.registry_root = HKEY_USERS;
        self
    }
    
    /// set to "HKEY_CURRENT_CONFIG" for searching the registry
    pub fn set_hkcc(&mut self) -> &mut Self
    {
        self.registry_root = HKEY_CURRENT_CONFIG;
        self
    }

    /// This lets you set any isize value, needs to be valid to work
    pub fn set_root(&mut self, reg_root: isize) -> &mut Self
    {
        self.registry_root = reg_root;
        self
    }

    /// Set the registry path that will act like our starting point for searching through the registry
    pub fn set_paths(&mut self, locations: Vec<String>) -> &mut Self
    {
        self.registry_paths = locations;
        self
    }
}


pub struct Applications
{
    app_paths: Vec<String>,
    reg_root: winreg::RegKey
}

impl Default for Applications
{
    fn default() -> Self {
        Self {
            app_paths: vec!["SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall".to_string(),"SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall".to_string()],
            reg_root: winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
        }
    }
}

impl Applications
{
    pub fn new() -> Self
    {
        Self::default()
    }
}