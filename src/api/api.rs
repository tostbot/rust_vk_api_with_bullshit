use super::session::Session;
use num::Num;
use reqwest::Response;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::IntoIterator;

pub struct VkApi {
    client: reqwest::Client,
    session: Session,
    version: String
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
            VkApiArg::Integer(I) => I.to_string(),
            VkApiArg::Float(F) => F.to_string(),
            VkApiArg::String(S) => S.to_string(),
            VkApiArg::Bool(B) => B.to_string(),
            _ => panic!("Not implemented")
        }
    }
}

impl VkApi {
    pub async fn call(&self, method: String, params: HashMap<String, VkApiArg>) {
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
        VkApi { session, client: reqwest::Client::new(), version: "5.91".to_string()}
    }
}
