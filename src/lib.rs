use schnorrkel::{
  vrf::{VRFInOut, VRFProof, VRFProofBatchable},
  *,
};
type DrawCardResult = (VRFInOut, VRFProof, VRFProofBatchable);

struct Player {
  drawed_card: Option<DrawCardResult>,
  key: Keypair,
}

// 1. Generate key pairs for each players
fn generate_key_pairs(number_of_players: u8) -> Vec<Player> {
  let change_me = Keypair::generate();
  // TODO: please change me
  vec![Player {
    drawed_card: None,
    key: change_me,
  }]
}

// 2. Take turn and draw card [x 5] for later
//   - VRF -> output % 52 = card number
//   - sign
fn draw_card(mut player: &Vec<Player>) {
  todo!("here we assign random value to the field `drawed_card`")
}

// 3. Determine the best output
fn find_best_player(all_players: &Vec<Player>) -> &Player {
    let mut best_player: Option<&Player> = None;
    let mut highest_card_value = 0;

    for player in all_players {
        if let Some((card, _, _)) = &player.drawed_card {
            let card_bytes = card.as_output_bytes();
            let card_sum: u32 = card_bytes.iter().map(|&b| b as u32).sum();
            let card_value = card_sum % 52;

            if card_value > highest_card_value {
                highest_card_value = card_value;
                best_player = Some(player);
            }
        }
    }

    best_player.expect("There should be at least one player with a drawn card")
}

fn verify_best_player(player: Player) -> bool {
  todo!()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn play_poker() {
    let player = Keypair::generate();
    // 3. Take turn and commit the output [do it inside for loop]
    // 5. Find the winner from the output
  }
}
