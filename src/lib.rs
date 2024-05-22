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
fn draw_card(players: &mut Vec<Player>) {
  let ctx = signing_context(b"Poker Game!");

  for one in players {
    one.drawed_card = Some(one.key.vrf_sign(ctx.bytes(b"I played")));
  }
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
  fn play_poker() {
    let player = Keypair::generate();
    // 3. Take turn and commit the output [do it inside for loop]
    // 5. Find the winner from the output
  }
}
