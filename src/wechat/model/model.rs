pub enum Currency{
    CNY,
}
pub trait ParamsTrait{
    fn to_json(&self) -> String;
}

#[derive(Debug)]
pub struct WechatPay {
    appid: String,
    mch_id: String,
    private_key: String,
    serial_no: String,
    v3_key: String,
    notify_url: String,
    base_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WechatPayDecodeData{
    pub mchid: String,
    pub appid: String,
    pub out_trade_no: String,
    pub transaction_id: String,
    pub trade_type: String,
    pub trade_state: String,
    pub trade_state_desc: String,
    pub bank_type: String,
    pub attach: String,
    pub success_time: String,
    pub payer: PayerInfo,
    pub amount: AmountInfo,
}

#[derive(Serialize, Debug, Clone)]
pub struct NativeParams{
       ///【商品描述】 商品描述
       pub description: String,
       ///【通知地址】 异步接收微信支付结果通知的回调地址，通知URL必须为外网可访问的URL，不能携带参数。 公网域名必须为HTTPS，如果是走专线接入，使用专线NAT IP或者私有回调域名可使用HTTP
       /// pub notify_url: String,
       ///【商户订单号】 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一。
       pub out_trade_no: String,
       ///【订单金额】 订单金额信息
       pub amount: AmountInfo,
       ///【交易结束时间】 订单失效时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
       #[serde(skip_serializing_if = "Option::is_none")]
       pub time_expire: Option<String>,
       ///【附加数据】 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
       #[serde(skip_serializing_if = "Option::is_none")]
       pub attach: Option<String>,
       ///【订单优惠标记】 商品标记，代金券或立减优惠功能的参数。
       #[serde(skip_serializing_if = "Option::is_none")]
       pub goods_tag: Option<String>,
       ///【电子发票入口开放标识】 传入true时，支付成功消息和支付详情页将出现开票入口。需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。
       #[serde(skip_serializing_if = "Option::is_none")]
       pub support_fapiao: Option<bool>,
       ///【场景信息】 支付场景描述
       #[serde(skip_serializing_if = "Option::is_none")]
       pub scene_info: Option<SceneInfo>,
       ///【结算信息】 结算信息
       #[serde(skip_serializing_if = "Option::is_none")]
       pub settle_info: Option<SettleInfo>,
}
// #[derive (Serialize, Debug, Clone)]
