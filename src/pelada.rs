const GOALKEEPER: [&str; 4] = ["goleiros", "goleiro", "gol", "arqueiros"];
const PLAYER: [&str; 3] = ["jogadores", "jogador", "linha"];
const GUEST: [&str; 8] = ["suplentes/convidados", "suplentes", "convidados", "convidado","suplência", "suplencia", "reservas", "reserva"];
const KID: [&str; 3] = ["sub 15", "menores", "menor"];

struct PeladaType {
    goalkeepers: Vec<String>,
    players: Vec<String>,
    guests: Vec<String>,
    kids: Vec<String>,
}

fn list_to_lower(list: Vec<String>) -> Vec<String> {
    list
        .iter()
        .map(|word| -> String {
            return word.to_lowercase();
        })
        .collect()
}

fn get_keyword(group: &[&str], vec: &Vec<String>) -> Option<String> {
    for key in group {
        let keyword: String = key.to_string();

        if vec.contains(&keyword) {
            return Some(keyword);
        }
    }
    None
}

pub fn from(list: Vec<String>) {
   let mut gk_keyword = String::from("");
   let mut pl_keyword = String::from("");
   let mut gt_keyword = String::from("");
   let mut kd_keyword = String::from("");

   //let mut gk = [""];
   //let mut pl = [""];
   //let mut gt = [""];
   //let mut kd = [""];

   let lowered_list: Vec<String> = list_to_lower(list); 

   match get_keyword(&GOALKEEPER, &lowered_list) {
       Some(keyword) => {
           gk_keyword = keyword;
       }
       None => {
           panic!("Please, check the list! I couldn't parse the goalkeeper keyword.");
       }
   } 
   
   match get_keyword(&PLAYER, &lowered_list) {
       Some(keyword) => {
           pl_keyword = keyword;
       }
       None => {
           panic!("Please, check the list! I couldn't parse the player keyword.");
       }
   } 
   
   match get_keyword(&GUEST, &lowered_list) {
       Some(keyword) => {
           gt_keyword = keyword;
       }
       None => {
           panic!("Please, check the list! I couldn't parse the guest keyword.");
       }
   } 
   
   match get_keyword(&KID, &lowered_list) {
       Some(keyword) => {
           kd_keyword = keyword;
       }
       None => {
           panic!("Please, check the list! I couldn't parse the kid keyword.");
       }
   } 

   println!("Keyword: {}", gk_keyword);
   println!("Keyword: {}", pl_keyword);
   println!("Keyword: {}", gt_keyword);
   println!("Keyword: {}", kd_keyword);
}
