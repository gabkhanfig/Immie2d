use std::{collections::{ HashSet, HashMap }, fmt};

use lazy_static::lazy_static;
use std::sync::Mutex;

struct GlobalStringMaps {
    map: HashMap<String, u32>,
    vec: Vec<String>,
    next_id: u32
}

lazy_static! {
    static ref GLOBAL_STRING_MAP: Mutex<GlobalStringMaps> = {
        let mut maps = GlobalStringMaps {
            map: HashMap::new(),
            vec: Vec::new(),
            next_id: 0
        };
        maps.map.insert("".to_string(), 0);
        maps.next_id = 1;
        maps.vec.push("".to_string());
        Mutex::new(maps)
    };

}

#[derive(Clone, Copy, PartialEq)]
pub struct GlobalString {
    string_id: u32 
}

impl GlobalString {   
    /// The default instance of GlobalString, which will contain an empty string
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::default();
    /// assert_eq!(gstr.to_string(), "".to_string());
    /// ```
    pub fn default() -> GlobalString {
        return GlobalString { 
            string_id: 0
        };
    }

    /// Will create a new GlobalString instance containing a pointer to the argument string
    /// that will get inserted into a global HashSet.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::new(&"hello world!".to_string());
    /// assert_eq!(gstr.to_string(), "hello world!".to_string());
    /// ```
    pub fn new(in_string: &String) -> GlobalString {
        //println!("Adding GlobalString {}", in_string);
        let mut maps = GLOBAL_STRING_MAP.lock().unwrap();
        let next_id = maps.next_id;
        let exists = maps.map.insert(in_string.clone(), next_id.clone());
        if exists.is_some() { // If the value already exists in the map, just use the existing id
            return GlobalString {
                string_id: exists.unwrap()
            };
        }
        maps.next_id += 1;
        maps.vec.push(in_string.clone());
        return GlobalString {
            string_id: next_id
        };
    }

    /// Will create a new GlobalString instance containing a pointer to the argument string
    /// ONLY if it already exists in the global HashSet. Is empty string otherwise.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::new_if_exists(&"some random string not in the map".to_string());
    /// assert_eq!(gstr.to_string(), "".to_string());
    /// ```
    pub fn new_if_exists(in_string: &String) -> GlobalString {
        let maps = GLOBAL_STRING_MAP.lock().unwrap();
        let exists: Option<&u32> = maps.map.get(in_string);
        if exists.is_none() {
            return GlobalString::default();
        }
        return GlobalString {
            string_id: exists.unwrap().clone()
        };  
    }

    /// Gets an copy to the String of the id held by GlobalString.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::new(&"hello world!".to_string());
    /// let ref_str: String = gstr.to_string();
    /// # assert_eq!(ref_str, "hello world!".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        let maps = GLOBAL_STRING_MAP.lock().unwrap();
        let as_string = &maps.vec[self.string_id as usize];
        return as_string.clone();
    }
}

impl fmt::Debug for GlobalString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GlobalString").field("internal_string",  &self.to_string()).finish()
    }
}

impl fmt::Display for GlobalString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self.to_string());
    }
}

