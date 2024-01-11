use serde::{Deserialize, Serialize};
use serde_json::Result;

const GOALKEEPER: [&str; 5] = ["goalkeepers", "goleiros", "goleiro", "gol", "arqueiros"];
const PLAYER: [&str; 4] = ["players", "jogadores", "jogador", "linha"];
const GUEST: [&str; 9] = [
    "guests",
    "suplentes/convidados",
    "suplentes",
    "convidados",
    "convidado",
    "suplência",
    "suplencia",
    "reservas",
    "reserva",
];
const KID: [&str; 5] = ["kids", "crianças", "sub 15", "menores", "menor"];

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

// Functions
// =======================================================================
fn list_to_lower(list: Vec<String>) -> Vec<String> {
    list.iter()
        .map(|word| -> String {
            return word.to_lowercase();
        })
        .collect()
}

fn empty_list_warning(keyword: &str) {
    println!(
        "Please, check the list! I couldn't parse the {} keyword. Assigning an empty list.",
        keyword
    );
}

fn get_keyword(group: &[&str], list: &Vec<String>) -> Option<String> {
    for key in group {
        let keyword: String = key.to_string();

        if list.contains(&keyword) {
            return Some(keyword);
        }
    }

    let group_name = match group.get(0) {
        Some(name) => name,
        None => "",
    };
    empty_list_warning(group_name);
    None
}

fn keywords(
    list: &Vec<String>,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let gk_keyword = get_keyword(&GOALKEEPER, &list);
    let pl_keyword = get_keyword(&PLAYER, &list);
    let gt_keyword = get_keyword(&GUEST, &list);
    let kd_keyword = get_keyword(&KID, &list);

    (gk_keyword, pl_keyword, gt_keyword, kd_keyword)
}

fn get_keyword_index(keyword_opt: &Option<String>, list: &Vec<String>) -> Option<usize> {
    match keyword_opt {
        Some(keyword) => {
            list.iter().position(|word| word == keyword)
            //match list.iter().position(|word| word == keyword) {
            //    Some(index) => {
            //        return index;
            //    }
            //    None => {
            //        panic!("keyword {} not found in the list!!", keyword);
            //    }
            //}
        }
        None => None,
    }
}

fn sublist(from: Option<usize>, to: Option<usize>, list: &Vec<String>) -> Vec<String> {
    match (from, to) {
        (Some(from_index), Some(to_index)) => return list[from_index..to_index].to_vec(),
        (Some(from_index), None) => return list[from_index..(list.len())].to_vec(),
        (None, _) => return [].to_vec(),
    }
}

pub fn from(list: Vec<String>) -> PeladaType {
    let lowered_list: Vec<String> = list_to_lower(list);

    let (gk_keyword, pl_keyword, gt_keyword, kd_keyword) = keywords(&lowered_list);

    let keywords_list: [Option<String>; 4] = [gk_keyword, pl_keyword, gt_keyword, kd_keyword];

    let indices: Vec<Option<usize>> = keywords_list
        .iter()
        .map(|keyword_opt| get_keyword_index(keyword_opt, &lowered_list))
        .collect();

    // TODO: Could be parallel
    let gk = sublist(Some(1), Some(2), &lowered_list);
    let pl = sublist(Some(2), Some(3), &lowered_list);
    let gt = sublist(Some(3), Some(4), &lowered_list);
    let kd = sublist(Some(4), None, &lowered_list);

    PeladaType {
        goalkeepers: gk,
        players: pl,
        guests: gt,
        kids: kd,
    }
}
