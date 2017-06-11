use std;
use std::collections::HashMap;
use std::fmt;



pub struct Postparams {
    params: HashMap<String, String>
}


#[allow(unused)]
impl Postparams {
    pub fn new() -> Self {
        Postparams {
            params: HashMap::new()
        }
    }


    pub fn add<T: ToString, B: ToString>(&mut self, key: T, value: B) -> &mut Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    pub fn remove<T: ToString>(&mut self, key: T) -> &mut Self {
        self.params.remove(&key.to_string());
        self
    }


    pub fn to_string(self) -> String {
        let params = self.params;
        let mut pairs = Vec::new();
        let urlencoded: String;


        for (key, value) in params {
            pairs.push(format!("{}={}", key, &value));
        }

        pairs.join("&")
    }
}


impl fmt::Display for Postparams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
