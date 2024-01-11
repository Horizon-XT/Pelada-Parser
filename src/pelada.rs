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

fn index_predicate(to: &usize, index_opt: Option<usize>) -> bool {
    match index_opt {
        Some(index) => return &index != to,
        None => false,
    }
}

fn sublist(
    from: usize,
    keywords_indices: &Vec<Option<usize>>,
    pelada_list: &Vec<String>,
) -> Vec<String> {
    let from_index: usize;

    // TODO Improve this...
    match keywords_indices.get(from) {
        Some(index_opt) => match index_opt {
            Some(index) => {
                from_index = *index;
            }
            None => return [].to_vec(),
        },
        None => return [].to_vec(),
    }

    let keywords_indices_len: usize = keywords_indices.len();
    let to: usize = from_index + 1;

    if to >= keywords_indices_len {
        return pelada_list[from_index..(pelada_list.len())].to_vec();
    }

    let binding = keywords_indices[to..keywords_indices_len].to_vec();
    let to_index_search_result = binding
        .iter()
        .find(|&&index_opt| index_predicate(&to, index_opt));

    match to_index_search_result {
        // TODO Improve this as well...
        Some(to_index_opt) => match to_index_opt {
            Some(to_index) => return pelada_list[from_index..*to_index].to_vec(),
            None => return pelada_list[from_index..(pelada_list.len())].to_vec(),
        },
        None => return pelada_list[from_index..(pelada_list.len())].to_vec(),
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
    let gk = sublist(0, &indices, &lowered_list);
    let pl = sublist(1, &indices, &lowered_list);
    let gt = sublist(2, &indices, &lowered_list);
    let kd = sublist(3, &indices, &lowered_list);

    PeladaType {
        goalkeepers: gk,
        players: pl,
        guests: gt,
        kids: kd,
    }
}
