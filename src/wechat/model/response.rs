use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct NativeResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    ///【支付跳转链接】 h5_url为拉起微信支付收银台的中间页面，可通过访问该URL来拉起微信客户端，完成支付，h5_url的有效期为5分钟。
    pub code_url: Option<String>,
}
pub trait ResponseTrait: DeserializeOwned {}