const GOALKEEPER: [&str; 4] = ["goleiros", "goleiro", "gol", "arqueiros"];
const PLAYER: [&str; 3] = ["jogadores", "jogador", "linha"];
const GUEST: [&str; 8] = ["suplentes/convidados", "suplentes", "convidados", "convidado","suplência", "suplencia", "reservas", "reserva"];
const KID: [&str; 3] = ["sub", "menores", "menor"];

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

fn get_keyword(group: &[&str], vec: Vec<String>) -> Option<String> {
    for key in group {
        let keyword: String = key.to_string();

        if vec.contains(&keyword) {
            return Some(keyword);
        }
    }
    None
}

pub fn from(list: Vec<String>) {

   let lowered_list: Vec<String> = list_to_lower(list); 

   match get_keyword(&GOALKEEPER, lowered_list) {
       Some(keyword) => {
           println!("The GK keyword: {}", keyword);
       }
       None => {
           println!("wtf");
       }
   } 
}
