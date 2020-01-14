use crate::api::api::VkApi;
use crate::api::api::VkApiArg;
use futures::executor::block_on;
use crate::api::session::Session;
#[macro_use] extern crate maplit;


mod api;


#[tokio::main]
async fn main() {

    let session = Session::from_token("cb55876e4a47f438d33fb456e170e695f7c5e8723668fe37e9251f3d338f5f79ecfa831de5cba96c01708");
    let api = VkApi::new(session);

    api.call("users.get".to_string(), hashmap!("user_ids".to_string() => VkApiArg::Integer(1))).await;

}
