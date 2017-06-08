use std::vec::Vec;
use api::entity;
use serde_json;


#[allow(unused)]
pub struct FullInfo {
    id:          u32,
    name:        String,
    genre:       Vec<String>,
    fsk:         Vec<String>,
    description: String,
    medium:      entity::medium::Medium,
    count:       u32,
    state:       entity::state::State,
    rate_sum:    u32,
    rate_count:  u32,
    clicks:      u32,
    category:    entity::category::Category,
    license:     entity::license::License,
    gate:        bool,
    names:       Vec<entity::name::Name>,
    lang:        Vec<String>,
    season:      Vec<entity::season::Season>,
    groups:      Vec<entity::group::Group>,
    publisher:   Vec<entity::publisher::Publisher>,
    tags:        Vec<entity::tag::Tag>,
}


impl FullInfo {
    pub fn new(data: serde_json::Value) {
        unimplemented!
    }
}
