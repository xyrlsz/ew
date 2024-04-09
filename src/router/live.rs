use json;
use json::object;
use json::JsonValue;
use crate::router::global;
use crate::encryption;
use actix_web::{HttpResponse, HttpRequest};
use crate::router::userdata;

pub fn retire(_req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": {
            "stamina": {},
            "item_list": [],
            "event_point_list": []
        }
    };
    global::send(resp)
}

pub fn guest(_req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": {"guest_list":[{"user":{"name":"A Nice Guest","comment":"Enjoy your live show!","exp":900,"main_deck_slot":1,"favorite_master_card_id":10010013,"favorite_card_evolve":0,"guest_smile_master_card_id":10010013,"guest_cool_master_card_id":10010013,"guest_pure_master_card_id":10010013,"friend_request_disabled":1,"master_title_ids":[3000001,0],"profile_settings":[1,2,3,4,5,6,7],"last_login_time":1708699449},"favorite_card":{"id":0,"master_card_id":10010013,"exp":1025,"skill_exp":0,"evolve":[]},"guest_smile_card":{"id":0,"master_card_id":10010013,"exp":1025,"skill_exp":0,"evolve":[]},"guest_cool_card":{"id":0,"master_card_id":10010013,"exp":1025,"skill_exp":0,"evolve":[]},"guest_pure_card":{"id":0,"master_card_id":10010013,"exp":1025,"skill_exp":0,"evolve":[]},"status":0}]}
    };
    global::send(resp)
}

pub fn mission(_req: HttpRequest, _body: String) -> HttpResponse {
    //todo
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": {
            "score_ranking": "",
            "combo_ranking": "",
            "clear_count_ranking": ""
        }
    };
    global::send(resp)
}

// /api/live/ranking
pub fn ranking(_req: HttpRequest, _body: String) -> HttpResponse {
    //todo
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": {
            "ranking_list": []
        }
    };
    global::send(resp)
}

pub fn start(_req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": []
    };
    global::send(resp)
}

pub fn update_live_data(user: &mut JsonValue, data: &JsonValue) -> JsonValue {
    if user["tutorial_step"].as_i32().unwrap() < 130 {
        return JsonValue::Null;
    }
    
    let mut rv = object!{
        "master_live_id": data["master_live_id"].as_i32().unwrap(),
        "level": data["level"].as_i32().unwrap(),
        "clear_count": 1,
        "high_score": data["live_score"]["score"].as_i32().unwrap(),
        "max_combo": data["live_score"]["max_combo"].as_i32().unwrap(),
        "auto_enable": 1, //whats this?
        "updated_time": global::timestamp()
    };
    
    let mut has = false;
    for (_i, current) in user["live_list"].members_mut().enumerate() {
        if current["master_live_id"].to_string() == rv["master_live_id"].to_string() {
            has = true;
            rv["clear_count"] = (current["clear_count"].as_i32().unwrap() + 1).into();
            current["clear_count"] = rv["clear_count"].clone();
            
            if rv["high_score"].as_i32().unwrap() > current["high_score"].as_i32().unwrap() {
                current["high_score"] = rv["high_score"].clone();
            } else {
                rv["high_score"] = current["high_score"].clone();
            }
            
            if rv["max_combo"].as_i32().unwrap() > current["max_combo"].as_i32().unwrap() {
                current["max_combo"] = rv["max_combo"].clone();
            } else {
                rv["max_combo"] = current["max_combo"].clone();
            }
            current["updated_time"] = rv["updated_time"].clone();
            break;
        }
    }
    if !has {
        user["live_list"].push(rv.clone()).unwrap()
    }
    rv
}

pub fn end(req: HttpRequest, body: String) -> HttpResponse {
    let key = global::get_login(req.headers(), &body);
    let body = json::parse(&encryption::decrypt_packet(&body).unwrap()).unwrap();
    let user2 = userdata::get_acc_home(&key);
    let mut user = userdata::get_acc(&key);
    
    user["stamina"]["stamina"] = (user["stamina"]["stamina"].as_i32().unwrap() - body["use_lp"].as_i32().unwrap()).into();
    if user["stamina"]["stamina"].as_i32().unwrap() < 0 {
        user["stamina"]["stamina"] = (0).into();
    }
    user["stamina"]["last_updated_time"] = global::timestamp().into();
    
    user["user"]["exp"] = (user["user"]["exp"].as_i32().unwrap() + body["use_lp"].as_i32().unwrap()).into();
    
    let live = update_live_data(&mut user, &body);
    
    userdata::save_acc(&key, user.clone());
    
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": {
            "item_list": user["item_list"].clone(),
            "point_list": user["point_list"].clone(),
            "live": live,
            "clear_master_live_mission_ids": [],
            "user": user["user"].clone(),
            "stamina": user["stamina"].clone(),
            "character_list": user["character_list"].clone(),
            "reward_list": [],
            "gift_list": user2["home"]["gift_list"].clone(),
            "clear_mission_ids": user2["clear_mission_ids"].clone(),
            "event_point_reward_list": [],
            "ranking_change": [],
            "event_member": [],
            "event_ranking_data": []
        }
    };
    global::send(resp)
}

pub fn clearrate(_req: HttpRequest) -> HttpResponse {
    
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": json::parse(include_str!("clearrate.json")).unwrap()
    };
    global::send(resp)
}
