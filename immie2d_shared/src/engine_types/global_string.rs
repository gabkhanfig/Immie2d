use std::{collections::HashSet, fmt};

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_STRING_SET: Mutex<HashSet<String>> = {
        let m = HashSet::new();
        Mutex::new(m)
    };
    static ref EMPTY_STRING: String = { 
        "".to_string() 
    };
}

#[derive(Clone, Copy)]
pub struct GlobalString {
    internal_string: *const String
}

impl GlobalString {   
    /// The default instance of GlobalString, which will contain an empty string
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::default();
    /// assert_eq!(gstr.to_string(), &"".to_string());
    /// ```
    pub fn default() -> GlobalString {
        return GlobalString { 
            internal_string: &*EMPTY_STRING as *const String
        };
    }

    /// Will create a new GlobalString instance containing a pointer to the argument string
    /// that will get inserted into a global HashSet.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::new(&"hello world!".to_string());
    /// assert_eq!(gstr.to_string(), &"hello world!".to_string());
    /// ```
    pub fn new(in_string: &String) -> GlobalString {
        let mut map = GLOBAL_STRING_SET.lock().unwrap();
        map.insert(in_string.clone());
        let mapped_string = map.get(in_string).unwrap();
        return GlobalString {
            internal_string: mapped_string as *const String
        };      
    }

    /// Will create a new GlobalString instance containing a pointer to the argument string
    /// ONLY if it already exists in the global HashSet. Is empty string otherwise.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::new_if_exists(&"some random string not in the map".to_string());
    /// assert_eq!(gstr.to_string(), &"".to_string());
    /// ```
    pub fn new_if_exists(in_string: &String) -> GlobalString {
        let map = GLOBAL_STRING_SET.lock().unwrap();
        let exists: Option<&String> = map.get(in_string);
        if exists.is_none() {
            return GlobalString::default();
        }
        return GlobalString {
            internal_string: exists.unwrap() as *const String
        };  
    }

    /// Gets an immutable reference to the String that the pointer held by GlobalString points to.
    /// ```
    /// use immie2d_shared::engine_types::global_string::GlobalString;
    /// let gstr = GlobalString::new(&"hello world!".to_string());
    /// let ref_str: &String = gstr.to_string();
    /// # assert_eq!(ref_str, &"hello world!".to_string());
    /// ```
    pub fn to_string(&self) -> &String {
        unsafe {
            &*self.internal_string
        }
    }
}

impl fmt::Debug for GlobalString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GlobalString").field("internal_string",  unsafe { &*self.internal_string}).finish()
    }
}

impl fmt::Display for GlobalString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", unsafe { &*self.internal_string});
    }
}

