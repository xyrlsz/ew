use json;
use json::{array, object};
use crate::router::global;
use crate::encryption;
use actix_web::{HttpResponse, HttpRequest, http::header::HeaderValue};
use crate::router::userdata;

pub fn user(req: HttpRequest) -> HttpResponse {
    let blank_header = HeaderValue::from_static("");
    
    let key = req.headers().get("a6573cbe").unwrap_or(&blank_header).to_str().unwrap_or("");
    let user = userdata::get_acc(key);
    
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": user
    };
    global::send(resp)
}

pub fn user_post(req: HttpRequest, body: String) -> HttpResponse {
    let body = json::parse(&encryption::decrypt_packet(&body).unwrap()).unwrap();
    let blank_header = HeaderValue::from_static("");
    
    let key = req.headers().get("a6573cbe").unwrap_or(&blank_header).to_str().unwrap_or("");
    let mut user = userdata::get_acc(key);
    let user_2 = userdata::get_acc_home(key);
    
    user["user"]["name"] = body["name"].clone();
    user["user"]["friend_request_disabled"] = body["friend_request_disabled"].clone();
    
    userdata::save_acc(key, user.clone());
    
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": {
            "user": user["user"].clone(),
            "clear_mission_ids": user_2["clear_mission_ids"].clone()
        }
    };
    global::send(resp)
}

pub fn initialize(req: HttpRequest, body: String) -> HttpResponse {
    let body = json::parse(&encryption::decrypt_packet(&body).unwrap()).unwrap();
    let blank_header = HeaderValue::from_static("");
    
    let key = req.headers().get("a6573cbe").unwrap_or(&blank_header).to_str().unwrap_or("");
    let mut user = userdata::get_acc(key);
    
    let id = (body["master_character_id"].as_i32().unwrap() * 10000) + 7; //todo - is this alwasy the case?
    user["user"]["favorite_master_card_id"] = id.into();
    user["user"]["guest_smile_master_card_id"] = id.into();
    user["user"]["guest_cool_master_card_id"] = id.into();
    user["user"]["guest_pure_master_card_id"] = id.into();
    
    let id = body["master_character_id"].to_string();
    let userr = &id[id.len() - 2..].parse::<i32>().unwrap();
    let mut masterid = 3000000;
    if id.starts_with("2") {
        masterid += 9; //muse
    } else if id.starts_with("3") {
        masterid += 9 + 9; //aquors
    } else if id.starts_with("4") {
        masterid += 9 + 9 + 12; //nijigasaki
    }
    masterid += userr;
    
    user["user"]["master_title_ids"][0] = masterid.into();
    
    // User is rewarded with all base cards in the team they chose. This makes up their new deck_list
    
    //nijigasaki for now
    let cardstoreward = array![30010001, 30020001, 30030001, 30050001, 30060001, 30070001, 30080001, 30090001, 30100001, 30110001];
    
    let ur = user["card_list"][user["card_list"].len() - 1]["id"].clone();
    //todo - does the user have the char already?
    for (i, data) in cardstoreward.members().enumerate() {
        let to_push = object!{
            "id": data.clone(),
            "master_card_id": data.clone(),
            "exp": 0,
            "skill_exp": 0,
            "evolve": [],
            "created_date_time": global::timestamp()
        };
        user["card_list"].push(to_push.clone()).unwrap();
        if i < 10 {
            user["deck_list"][0]["main_card_ids"][i] = data.clone();
        }
    }
    user["deck_list"][0]["main_card_ids"][4] = ur;
    
    userdata::save_acc(key, user.clone());
    
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": user["user"].clone()
    };
    global::send(resp)
}
