use serde::{Deserialize, Serialize};
use serde_json::Result;

const GOALKEEPER: [&str; 4] = ["goleiros", "goleiro", "gol", "arqueiros"];
const PLAYER: [&str; 3] = ["jogadores", "jogador", "linha"];
const GUEST: [&str; 8] = [
    "suplentes/convidados",
    "suplentes",
    "convidados",
    "convidado",
    "suplência",
    "suplencia",
    "reservas",
    "reserva",
];
const KID: [&str; 3] = ["sub 15", "menores", "menor"];

#[derive(Serialize, Deserialize)]
pub struct PeladaType {
    pub goalkeepers: Vec<String>,
    pub players: Vec<String>,
    pub guests: Vec<String>,
    pub kids: Vec<String>,
}

impl PeladaType {
    pub fn to_json(&self) -> Result<String> {
        let j = serde_json::to_string(&self)?;

        Ok(j)
    }
}

fn list_to_lower(list: Vec<String>) -> Vec<String> {
    list.iter()
        .map(|word| -> String {
            return word.to_lowercase();
        })
        .collect()
}

fn get_keyword(group: &[&str], list: &Vec<String>) -> Option<String> {
    for key in group {
        let keyword: String = key.to_string();

        if list.contains(&keyword) {
            return Some(keyword);
        }
    }
    None
}

fn keywords(list: &Vec<String>) -> (String, String, String, String) {
    let gk_keyword: String;
    let pl_keyword: String;
    let gt_keyword: String;
    let kd_keyword: String;

    match get_keyword(&GOALKEEPER, &list) {
        Some(keyword) => {
            gk_keyword = keyword;
        }
        None => {
            panic!("Please, check the list! I couldn't parse the goalkeeper keyword.");
        }
    }

    match get_keyword(&PLAYER, &list) {
        Some(keyword) => {
            pl_keyword = keyword;
        }
        None => {
            panic!("Please, check the list! I couldn't parse the player keyword.");
        }
    }

    match get_keyword(&GUEST, &list) {
        Some(keyword) => {
            gt_keyword = keyword;
        }
        None => {
            panic!("Please, check the list! I couldn't parse the guest keyword.");
        }
    }

    match get_keyword(&KID, &list) {
        Some(keyword) => {
            kd_keyword = keyword;
        }
        None => {
            panic!("Please, check the list! I couldn't parse the kid keyword.");
        }
    }

    (gk_keyword, pl_keyword, gt_keyword, kd_keyword)
}

fn get_keyword_index(keyword: &String, list: &Vec<String>) -> usize {
    match list.iter().position(|word| word == keyword) {
        Some(index) => {
            return index;
        }
        None => {
            panic!("keyword {} not found in the list!!", keyword);
        }
    }
}

fn sublist(from: &String, to: &String, list: &Vec<String>) -> Vec<String> {
    let from_index: usize = get_keyword_index(from, list) + 1;

    if to.is_empty() {
        return list[from_index..(list.len())].to_vec();
    } else {
        let to_index: usize = get_keyword_index(to, list);

        return list[from_index..to_index].to_vec();
    }
}

pub fn from(list: Vec<String>) -> PeladaType {
    let lowered_list: Vec<String> = list_to_lower(list);

    let (gk_keyword, pl_keyword, gt_keyword, kd_keyword) = keywords(&lowered_list);

    // TODO: Could be parallel
    let gk = sublist(&gk_keyword, &pl_keyword, &lowered_list);
    let pl = sublist(&pl_keyword, &gt_keyword, &lowered_list);
    let gt = sublist(&gt_keyword, &kd_keyword, &lowered_list);
    let kd = sublist(&kd_keyword, &String::from(""), &lowered_list);

    PeladaType {
        goalkeepers: gk,
        players: pl,
        guests: gt,
        kids: kd,
    }
}
