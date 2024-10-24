use crate::model::{NativeParams,ParamsTrait};
use crate::model::ResponseTrait;
use crate::wechat::error::PayError;
use crate::model::HttpMethod;


impl WechatPay{
    //包内使用函数
    pub(crate) fn pay<P: ParamsTrait, R: ResponseTrait>(
        &self,
        method: HttpMethod,
        url: &str,
        json: P,
    ) -> Result<R, PayError> {
        let json_str = json.to_json();
        let mut map: Map<String,Value> = serde_json::from_str(&json_str)?;
        map.insert("appid".to_owned(),self.appid().into());
        map.insert("mch_id".to_owned(),self.mch_id.into());
        map.insert("notify_url".to_owned(),self.notify_url.into());
        // 构建body
        let body = serde_json::to_string(&map)?;
        // 构建headers
        let headers = self.build_headers(method.clone(),url,body.as_str())?;
        let client = reqwest::blocking::Client::new(); // 创建同步客户端
        let url = format!("{}{}",self.base_url(),url);

        let builder = match method {
            HttpMethod::GET  => client.get(&url),
            HttpMethod::POST => client.post(&url),
            HttpMethod::PUT  => client.put(&url),
            HttpMethod::DELETE => client.delete(&url),
            HttpMethod::PATCH => client.patch(&url),
        };

        builder
            .headers(headers)
            .body(body)
            .send()?
            .json::<R>()?
            .map(Ok)?

    }
    pub fn native_pay(&self,params:NativeParams)-> Result<NativeResponse, PayError>{
        let url = "/v3/pay/transactions/native";
        self.pay(HttpMethod::POST,url,params)
    }
}