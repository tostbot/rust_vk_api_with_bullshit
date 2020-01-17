use crate::api::api::VkApi;
use crate::api::api::{VkApiArg, VkApiType};
use crate::api::longpoll::LongPoll;
use futures::executor::block_on;
use crate::api::session::Session;
use serde_json::Value;

#[macro_use] extern crate maplit;



#[macro_use] mod api;

#[tokio::main]
async fn main() {

    let session = Session::from_token("abacaba");
    let api = VkApi::new(session);

    let value = api.call("users.get", vk_args!("user_ids" => vec!(1, 2))).await;
    println!("Resp: {:?}", value.get("response"));

    let mut longpoll = LongPoll::with_pts(api);
    longpoll.update_server().await;
    longpoll.get_events().await;

}
