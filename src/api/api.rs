use super::session::Session;
use num::Num;
use reqwest::Response;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::IntoIterator;

pub struct VkApi {
    client: reqwest::Client,
    session: Session,
    version: String,
}

pub enum VkApiArg {
    Integer(i64),
    Float(f32),
    String(String),
    Bool(bool),

    IntArray(Vec<i64>),
    FloatArray(Vec<f32>),
    StringArray(Vec<String>),
    BoolArray(Vec<bool>),
}

impl VkApiArg {
    fn serialize(&self) -> String {
        match &self {
            VkApiArg::Integer(i) => i.to_string(),
            VkApiArg::Float(f) => f.to_string(),
            VkApiArg::String(s) => s.to_string(),
            VkApiArg::Bool(b) => b.to_string(),
            VkApiArg::IntArray(ia) => {
                let strings: Vec<String> = ia.iter().map(|x| x.to_string()).collect();
                strings.join(",")
            }
            _ => panic!("Not implemented")
        }
    }
}

impl VkApi {
    pub async fn call(&self, method: &str, params: HashMap<String, VkApiArg>) {
        let mut params_map: HashMap<String, String> = HashMap::new();

        params_map.insert("access_token".to_string(), self.session.token());
        params_map.insert("v".to_string(), self.version.clone());


        for (key, value) in params {
            params_map.insert(key, value.serialize());
        }

        let res = self.client
            .post(&format!("https://api.vk.com/method/{}", method))
            .form(&params_map)
            .send().await;

        let resp: Response = res.unwrap();

        println!("Status: {}", resp.text().await.unwrap())
    }

    pub fn new(session: Session) -> Self {
        VkApi { session, client: reqwest::Client::new(), version: "5.91".to_string() }
    }
}

pub trait VkApiType {
    fn get_enum_type(&self) -> VkApiArg;
}

impl VkApiType for i64 {
    fn get_enum_type(&self) -> VkApiArg {
        VkApiArg::Integer(*self)
    }
}

impl VkApiType for Vec<i64> {
    fn get_enum_type(&self) -> VkApiArg {
        VkApiArg::IntArray(self.clone())
    }
}
