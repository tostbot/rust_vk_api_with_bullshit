use crate::api::api::VkApi;
use crate::api::api::VkApiArg;
use crate::api::session::Session;
#[macro_use] extern crate maplit;


mod api;

fn main() {
    let session = Session::from_token("abacaba".to_string());
    let api = VkApi::new(session);
    api.call("users.get".to_string(), hashmap!("user_ids".to_string() => Box::new(1 as VkApiArg)));
}
