use std::env::Args;
use std::iter::{Skip};
use std::io::Error;
use std::collections::HashMap;

fn main() {
    let mut args: Skip<Args> = std::env::args().skip(1);
    let key: String = args.next().expect("There was no key supplied.");
    let value: String = args.next().expect("You didn't supply a value");
    println!("The key is {0:?} and the value is {1:?}", key, value);
    let contents: String = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).expect("Something went wrong with writing to a file");
    let database: Database = Database::new().expect("Database::new() crashed");
}

struct Database {
    map: HashMap<String, String>
}

impl Database {
    fn new() -> Result<Database, Error> {
        /*let contents= match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Result::Err(error);
            }
        };*/
        let mut map = HashMap::new();
        //read the kv.db file
        let contents = std::fs::read_to_string("kv.db")?;
        //parse the string
        for line in contents.lines(){
            let mut chunks =line.splitn(2,'\t');
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
            //populate the hashmap
            map.insert(key.to_owned(),value.to_owned());
        }

        return Result::Ok(Database {
            map
        });
    }
}