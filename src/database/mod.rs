pub struct History
{
    history: rusqlite::Connection
}

impl History
{
    pub fn new() -> Self
    {
        let _ = std::fs::create_dir(format!("{}/.tabletop", crate::env::get_home().unwrap()));

        let file_path = format!("{}/.tabletop/history.db3", crate::env::get_home().unwrap());

        let conn = rusqlite::Connection::open(&file_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id    INTEGER PRIMARY KEY,
                datetime  TEXT NOT NULL,
                action  TEXT NOT NULL
            )",
            (),
        ).unwrap();

        


        History { 
            history: conn
        }
    }

    pub fn add_entry(&mut self, entry: &str)
    {
        self.history.execute(
            "INSERT INTO history (datetime, action) VALUES (?1, ?2)", 
            (chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), entry)
        ).unwrap();
    }

    pub fn get_history_full(&mut self) -> Vec<String>
    {
        let mut vector: Vec<String> = Vec::new();
        let mut statement = self.history.prepare("SELECT * FROM history ORDER BY id DESC").unwrap();
        
        
        let history_iter = statement.query_map([], |row| {
            Ok(DB_OUT {
                id: row.get(0)?,
                datetime: row.get(1)?,
                action: row.get(2)?,
            })
        }).unwrap();

        for history_item in history_iter
        {
            if history_item.is_err() {continue;}
            let history = history_item.unwrap();
            vector.push(format!("{} | {}", history.datetime, history.action));
            
        }

        vector
    }

    pub fn get_history_last_x(&mut self, num_entries: u64) -> Vec<String>
    {
        let mut vector: Vec<String> = Vec::new();
        let mut statement = self.history.prepare(&format!("SELECT * FROM history ORDER BY id DESC LIMIT {}", num_entries)).unwrap();
        
        
        let history_iter = statement.query_map([], |row| {
            Ok(DB_OUT {
                id: row.get(0)?,
                datetime: row.get(1)?,
                action: row.get(2)?,
            })
        }).unwrap();

        for history_item in history_iter
        {
            if history_item.is_err() {continue;}
            let history = history_item.unwrap();
            vector.push(format!("{} | {}", history.datetime, history.action));
            
        }

        vector
    }

    pub fn clear_all(&mut self)
    {
        let _ = self.history.execute("DELETE FROM history WHERE 1=1", []);
    }


}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn home_dir() 
    {
        
    }
}

#[allow(non_camel_case_types)]
struct DB_OUT
{
    #[allow(dead_code)]
    id: i32,
    datetime: String,
    action: String,

}