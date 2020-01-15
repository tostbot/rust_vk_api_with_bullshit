#[macro_use] use super::api;
use super::api::{VkApi, VkApiArg, VkApiType};
use serde_json::to_string;

pub struct LongPoll{
    api: VkApi,
    is_connected: bool,
    client: reqwest::Client,
    need_pts: bool,
    ts: Option<u64>,
    pts: Option<u64>,
    server: Option<String>
}

macro_rules! vk_args {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(vk_arg!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { vk_arg!($($key => $value),+) };
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

impl LongPoll{
    pub fn new(api: VkApi, need_pts: bool) -> Self{
        return LongPoll{
            api,
            is_connected: false,
            client: reqwest::Client::new(),
            need_pts,
            ts: None,
            pts: None,
            server: None
        }
    }

    pub fn with_pts(api: VkApi) -> Self{
        Self::new(api, true)
    }

    pub async fn update_server(&mut self){
        let resp = self.api.call("messages.getLongPollServer", vk_args!("lp_version" => 3, "need_pts" => self.need_pts)).await;
        let response = &resp["response"];
        let ts = &response["ts"].as_u64();
        let server = response["server"].as_str().unwrap().to_string();

        if self.need_pts{
            let pts = &resp["pts"].as_u64();
            self.pts = *pts;
        }


        self.ts = *ts;
        self.server = Some(server);

        println!("Server updated. Server: {server:?}, ts: {ts:?}, pts: {pts:?}", server=self.server, ts=ts, pts=self.pts)

    }

    fn connect(){

    }

}

