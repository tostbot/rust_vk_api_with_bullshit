use crate::api::api::VkApi;
use crate::api::api::{VkApiArg, VkApiType};
use futures::executor::block_on;
use crate::api::session::Session;
#[macro_use] extern crate maplit;


mod api;

macro_rules! vk_args {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key.to_string(), $value.get_enum_type());
            )*
            _map
        }
    };
}


#[tokio::main]
async fn main() {

    let session = Session::from_token("abacaba");
    let api = VkApi::new(session);

    api.call("users.get", vk_args!("user_ids" => vec!(1, 2))).await;

}
