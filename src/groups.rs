use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub light_ids: Vec<u32>,
    pub state: GroupState,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupState {
    pub all_on: bool,
    pub any_on: bool,
}

#[derive(Deserialize)]
struct GetGroupResponseElement {
    pub name: String,
    pub lights: Vec<String>,
    pub r#type: String,
    pub state: GroupState,
    pub class: String
}

impl Group {

    pub fn from_json(json: String) -> Result<Vec<Group>, ()> {
        let groups: BTreeMap<u32, GetGroupResponseElement> = serde_json::from_str(json.as_str()).unwrap();
        let mut parsed_groups: Vec<Group> = vec![];
        for (group_id, group) in groups {
            let light_ids_in_group: Vec<u32> = group.lights.into_iter().map(|id| id.parse::<u32>().unwrap() ).collect();
            parsed_groups.push(Group{id: group_id, name: group.name.clone(), light_ids: light_ids_in_group, state: group.state});
        }
        Ok(parsed_groups)
    }
}