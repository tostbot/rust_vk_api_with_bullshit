use std::collections::HashMap;
use num::Num;
use super::session::Session;
use std::iter::IntoIterator;
use reqwest::Response;
use std::fmt::Display;

pub struct VkApi{
    session: Session,
}

impl VkApi{
    pub(crate) fn new(session: Session) -> Self{
        VkApi{
            session
        }
    }
}

pub trait VkApiArg{
    fn serialize(&self) -> String;
}

impl<T: Num + Display> VkApiArg for T {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl VkApi{
    pub async fn call(&self, method: String, params: HashMap<String, Box<dyn VkApiArg>>){
        let client = reqwest::Client::new();

        let mut params_map: HashMap<String, String> = HashMap::new();



        for (key, value) in params{
            params_map.insert(key, value.serialize());
        }


        let res = client.post("https://api.vk.com")
            .form(&params_map)
            .send()
            .await;

        let resp: Response = res.unwrap();

        println!("{}", resp.status())

    }
}


