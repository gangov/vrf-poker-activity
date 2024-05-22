use schnorrkel::{
  vrf::{VRFInOut, VRFProof, VRFProofBatchable},
  *,
};
type DrawCardResult = (VRFInOut, VRFProof, VRFProofBatchable);

#[derive(Debug)]
struct Player {
  pub drawed_card: Option<DrawCardResult>,
  pub key: Keypair,
}

// 1. Generate key pairs for each players
fn generate_key_pairs(number_of_players: u8) -> Vec<Player> {
  let mut players: Vec<Player> = Vec::with_capacity(number_of_players.into());

  for _ in 0..number_of_players {
    let key = Keypair::generate();
    println!("{:?}", &key.public);
    let player = Player {
      drawed_card: None,
      key,
    };
    players.push(player);
  }
  players
}

// 2. Take turn and draw card [x 5] for later
//   - VRF -> output % 52 = card number
//   - sign
fn draw_card(mut player: &Vec<Player>) {
  todo!("here we assign random value to the field `drawed_card`")
}

// 4. Determine the best output
fn find_best_player(all_players: &Vec<Player>) -> Player {
  todo!()
}

fn verify_best_player(player: Player) -> bool {
  todo!()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_generate_key_pairs() {
    let players = generate_key_pairs(4);
    assert_eq!(players.len(), 4);
  }

  #[test]
  fn play_poker() {
    let player = Keypair::generate();
    // 3. Take turn and commit the output [do it inside for loop]
    // 5. Find the winner from the output
  }
}
