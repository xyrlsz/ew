use json::{object, array, JsonValue};
use actix_web::{HttpRequest};


//todo
pub fn reward(req: HttpRequest) -> Option<JsonValue> {
    
    Some(object!{
        "reward_list": []
    })
}

pub fn reward_post(req: HttpRequest, _body: String) -> Option<JsonValue> {
    Some(array![])
}
