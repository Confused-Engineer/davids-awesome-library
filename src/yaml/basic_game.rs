const YAML_FILE: &str = "gamedata.yaml";
#[allow(dead_code)]
const YAML_TEST_FILE: &str = "./test.yaml";
const GAME_VERSION: f64 = 0.9;

pub struct GameData
{
   file: String,
   data: Data,
}

impl GameData {
    pub fn init() -> Self
    {
        
        GameData { 
            file: Self::file_dir(), 
            data: Self::load_data(),  
        }

    }

    pub fn game_list(&mut self) -> Vec<String>
    {
        let mut list: Vec<String> = Vec::new();
        for item in self.data.game.iter()
        {
            list.push(item.name.clone());
        }
        list
    }

    pub fn get_game(&mut self, title: &str) -> Result<Game, std::io::Error>
    {
        for x in 0..self.data.game.len()
        {
            if self.data.game[x].name == title
            {
                return Ok(self.data.game[x].clone())
                
            }
        }

        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Game not found"))
    }

    pub fn add_game(&mut self, game: Game)
    {
        let name = game.name.clone();
        let index = self.data.game
        .iter()
        .position(|pos| pos.name == name);

        if index.is_some()
        {
            self.data.game[index.unwrap()] = game;
        } else {
            self.data.game.push(game);
        }
    }

    pub fn remove_game(&mut self, title: &str)
    {
        let temp = self.data.game
        .iter()
        .position(|pos| pos.name == title);

        if temp.is_none()
        {
            return;
        }

        self.data.game.remove(temp.unwrap());
    }

    pub fn save_data(&mut self)
    {
        let file = std::fs::File::options().write(true).create(true).open(Self::file_dir()).unwrap();
        let serde_val = serde_yaml::to_value(&self.data).unwrap();
        let _ = serde_yaml::to_writer(file, &serde_val);
    }

    pub fn get_path(&mut self) -> String
    {
        self.file.clone()
    }

    pub fn get_version(&mut self) -> f64
    {
        self.data.version.clone()
    }

    fn load_data() -> Data
    {
        let valid = Self::validation_check();
        if valid.is_err()
        {
            return Data::default()
        }

        valid.unwrap()
    }

    fn validation_check() -> Result<Data, serde_yaml::Error>
    {
        let file = std::fs::read_to_string(Self::file_dir()).unwrap_or(String::new());
        let game: Data = serde_yaml::from_str::<Data>(&file)?;
        Ok(game)
    }

    fn file_dir() -> String
    {
        let home_dir = crate::env::get_home();
        if home_dir.is_err()
        {
            #[allow(unused_variables)]
            let home_dir = std::env::current_dir().unwrap().to_str();
        }

        let home_dir = String::from(home_dir.unwrap());
       
        let _ = std::fs::create_dir(format!("{}/.tabletop", home_dir));
        let file_dir = format!("{}/.tabletop/{}", home_dir, YAML_FILE);
        file_dir
    }
}





use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    game: Vec<Game>,
    version: f64
}

impl Default for Data
{
    fn default() -> Self {
        Self { 
            game: Vec::new(), 
            version: GAME_VERSION 
        }
    }


}




#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Game
{
    name: String,
    players: Vec<Player>,
    shops: Vec<Shop>,
}

impl Game {
    pub fn new() -> Self
    {
        Game { 
            name: String::new(), 
            players: Vec::new(), 
            shops: Vec::new() 
        }
    }

    pub fn get_instance(&mut self) -> Self
    {
        self.clone()
    }

    pub fn add_player(&mut self, player: Player)
    {
        if player.name.is_empty()
        {
            return;
        }

        let temp_pos = self.players
        .iter()
        .position(|pos| pos.name == player.name);

        if temp_pos.is_none()
        {
            self.players.push(player);
        } else {
            self.players[temp_pos.unwrap()] = player;
        }
        
    }

    pub fn get_single_player(&mut self, player_name: &str) -> Player
    {
        let temp = self.players
        .iter()
        .position(|pos| pos.name == player_name)
        .unwrap();

        self.players[temp].clone()
    }

    pub fn get_players(&mut self) -> Vec<Player>
    {
        self.players.clone()
    }

    pub fn remove_player(&mut self, player_name: &str)
    {
        let temp = self.players
        .iter()
        .position(|pos| pos.name == player_name)
        .unwrap();
    

        self.players.remove(temp);
    }

    pub fn add_shop(&mut self, shop: Shop)
    {
        

        if shop.name.is_empty()
        {
            return;
        }
        
        let temp_pos = self.shops
        .iter()
        .position(|pos| pos.name == shop.name);

        if temp_pos.is_none()
        {
            self.shops.push(shop);
        } else {
            self.shops[temp_pos.unwrap()] = shop;
        }
    }

    pub fn remove_shop(&mut self, shop_name: &str)
    {
        let temp = self.shops
        .iter()
        .position(|pos| pos.name == shop_name)
        .unwrap();
    

        self.shops.remove(temp);
    }

    pub fn get_single_shop(&mut self, shop_name: &str) -> Shop
    {
        let temp = self.shops
        .iter()
        .position(|pos| pos.name == shop_name)
        .unwrap();

        self.shops[temp].clone()
    }

    pub fn get_shops(&mut self) -> Vec<Shop>
    {
        self.shops.clone()
    }
}

impl Name for Game
{
    fn set_name(&mut self, name: &str)
    {
        self.name = name.to_string();
    }

    fn get_name(&mut self) -> String
    {
        self.name.clone()
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Player
{
    name: String,
    stats: std::collections::BTreeMap<String, String>,
    inventory: std::collections::BTreeMap<String, i64>,
}

impl Player
{
    pub fn new() -> Self
    {
        Player { 
            name: String::new(), 
            stats: std::collections::BTreeMap::new(), 
            inventory: std::collections::BTreeMap::new() 
        }
    }

    pub fn get_instance(&mut self) -> Self
    {
        self.clone()
    }

    pub fn get_stats(&mut self) -> std::collections::BTreeMap<String, String> {
        self.stats.clone()
    }

    pub fn set_stat(&mut self, key: String, value: String) {
        if key.is_empty()
        {
            return;
        }
        self.stats.insert(key, value);
    }

    pub fn remove_stat(&mut self, key: String) {
        self.stats.remove(&key);
    }
}

impl Name for Player
{
    fn set_name(&mut self, name: &str)
    {
        self.name = name.to_string();
    }

    fn get_name(&mut self) -> String
    {
        self.name.clone()
    }
}



impl Inventory for Player {
    fn get_inventory(&mut self) -> std::collections::BTreeMap<String, i64> {
        self.inventory.clone()
    }

    fn set_item(&mut self, key: String, value: i64) {
        if key.is_empty()
        {
            return;
        }
        self.inventory.insert(key, value);
    }

    fn remove_item(&mut self, key: String) {
        self.inventory.remove(&key);
    }
}



#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Shop
{
    name: String,
    inventory: std::collections::BTreeMap<String, i64>,
}


impl Shop
{
    pub fn new() -> Self
    {
        Shop { 
            name: String::new(), 
            inventory: std::collections::BTreeMap::new() 
        }
    }

    pub fn get_instance(&mut self) -> Self
    {
        self.clone()
    }


}

impl Name for Shop
{
    fn set_name(&mut self, name: &str)
    {
        self.name = name.to_string();
    }

    fn get_name(&mut self) -> String
    {
        self.name.clone()
    }
}


impl Inventory for Shop
{
    fn get_inventory(&mut self) -> std::collections::BTreeMap<String, i64> {
        self.inventory.clone()
    }

    fn set_item(&mut self, key: String, value: i64) {
        if key.is_empty()
        {
            return;
        }
        self.inventory.insert(key, value);
    }

    fn remove_item(&mut self, key: String) {
        self.inventory.remove(&key);
    }
}



pub trait Name {
    fn get_name(&mut self) -> String;
    fn set_name(&mut self, name: &str);
}

pub trait Inventory {
    fn get_inventory(&mut self) -> std::collections::BTreeMap<String, i64>;
    fn set_item(&mut self, key: String, value: i64);
    fn remove_item(&mut self, key: String);
}




#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn home_dir() 
    {

        println!("{}", crate::yaml::basic_game::GameData::file_dir());
        
        
    }


}
