use schnorrkel::{
  context::SigningContext,
  vrf::{VRFInOut, VRFProof, VRFProofBatchable},
  *,
};
type DrawCardResult = (VRFInOut, VRFProof, VRFProofBatchable);

struct Poker {
  players: Vec<Player>,
  input: Option<u32>,
  signing_ctx: SigningContext,
}

impl Poker {}

#[derive(Debug)]
struct Player {
  random_numer: Option<u32>,
  proof: Option<[u8; 32]>,
  pub drawed_card: Option<DrawCardResult>,
  key: Keypair,
}

impl Player {
  // fn new() -> Self {

  // }
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
      random_numer: None,
      proof: None,
    };
    players.push(player);
  }
  players
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
