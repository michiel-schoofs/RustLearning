use std::env::Args;
use std::iter::{Skip};
use std::io::Error;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let mut args: Skip<Args> = std::env::args().skip(1);
    let key: String = args.next().expect("There was no key supplied.");
    let value: String = args.next().expect("You didn't supply a value");
    println!("The key is {0:?} and the value is {1:?}", key, value);
    let mut database: Database = Database::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key,value);
    database.flush().expect("Something went wrong writing the database to disk");
}

struct Database {
    map: HashMap<String, String>,
    flush: bool
}

impl Database {
    fn new() -> Result<Database, Error> {
        //check if file exist if not create new file
        if !Path::new("kv.db").exists() {
            println!("The file does not exist creating database file");
            std::fs::File::create("kv.db").expect("Something went wrong writing the file");
        }

        let mut map = HashMap::new();
        //read the kv.db file
        /*let contents= match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Result::Err(error);
            }
        };*/
        let contents = std::fs::read_to_string("kv.db")?;
        //parse the string
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
            //populate the hashmap
            map.insert(key.to_owned(), value.to_owned());
        }

        return Result::Ok(Database {
            map,
            flush: false
        });
    }

    fn insert(&mut self,key: String, value:String) -> (){
        self.map.insert(key,value);
    }

    fn flush(mut self) -> Result<(), std::io::Error>{
        self.flush = true;
        return flush(&self);
    }
}

impl Drop for Database{
    fn drop (&mut self){
        if !self.flush {
            let _ = flush(self);
        }
    }
}

fn flush(database: &Database) -> Result<(), std::io::Error>{
    let mut contents:String = String::new();
    for (key,value) in &database.map{
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }

    return std::fs::write("kv.db", contents);
}