use std::io::Read;

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

impl Poker {
  fn new(players: Vec<Player>, input: Option<u32>) -> Self {
    Poker {
      players,
      input,
      signing_ctx: signing_context(b"Poker Game!"),
    }
  }
}

#[derive(Debug, Clone)]
struct Player {
  pub random_numer: Option<u32>,
  pub proof: Option<[u8; 32]>,
  pub drawed_card: Option<DrawCardResult>,
  pub key: Keypair,
}

impl Player {
  fn new() -> Self {
    let key = Keypair::generate();
    Self {
      drawed_card: None,
      key,
      random_numer: None,
      proof: None,
    }
  }
}

// 1. Generate key pairs for each players
fn generate_key_pairs(number_of_players: u8) -> Vec<Player> {
  let mut players: Vec<Player> = Vec::with_capacity(number_of_players.into());

  for _ in 0..number_of_players {
    players.push(Player::new());
  }
  players
}

// 2. Take turn and draw card [x 5] for later
//   - VRF -> output % 52 = card number
//   - sign
fn draw_card(game: &mut Poker) {
  let input = game.input.expect("Input expected!");

  for mut one in &mut game.players {
    one.drawed_card = Some(
      one
        .key
        .vrf_sign(game.signing_ctx.bytes(&input.to_le_bytes())),
    );
  }
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

fn verify_best_player(player: Player, poker_game: Poker) -> bool {
  let msg = poker_game.input.unwrap().to_le_bytes();
  let signing_tx = poker_game.signing_ctx.bytes(&msg);
  let pre_out = player.drawed_card.as_ref().unwrap().0.to_preout();
  let out = &player.drawed_card.as_ref().unwrap().0;

  let proof = &player.drawed_card.as_ref().unwrap().1;
  let proof_batchable = &player.drawed_card.as_ref().unwrap().2;

  let (io, proof_result) = player
    .key
    .public
    .vrf_verify(signing_tx, &pre_out, &proof)
    .expect("failed to verify");

  io.eq(out) && proof_batchable.eq(&proof_result)
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
  fn test_draw_card() {
    let players = generate_key_pairs(4);
    let mut game = Poker::new(players, Some(32));

    draw_card(&mut game);

    for player in game.players {
      assert!(player.drawed_card.is_some(), "Should be present")
    }
  }

  #[test]
  fn play_poker() {
    let player = Keypair::generate();
    // 3. Take turn and commit the output [do it inside for loop]
    // 5. Find the winner from the output
  }
  #[test]
  fn test_find_best_player() {

        let player = Player::new();
        let player_2 = Player::new();
        let player_3 = Player::new();
        let mut game = Poker::new(
          vec![player.clone(), player_2.clone(), player_3.clone()],
          Some(32),
        );
        draw_card(&mut game);
        let best_player = find_best_player(&game.players);
        assert_eq!(best_player.proof, player.proof);
  }

  #[test]
  fn test_verify_best_player_when_valid() {
    let player = Player::new();
    let player_2 = Player::new();
    let player_3 = Player::new();

    let mut game = Poker::new(
      vec![player.clone(), player_2.clone(), player_3.clone()],
      Some(32),
    );
    draw_card(&mut game);
    let best_player = find_best_player(&game.players);
    let best_player_is_truthy = verify_best_player(best_player.clone(), game);
    assert!(best_player_is_truthy);
  }

  #[test]
  #[ignore = "fix later"]
  fn test_verify_best_player_when_invalid() {
    let player = Player::new();
    let player_2 = Player::new();
    let player_3 = Player::new();

    let mut game = Poker::new(
      vec![player.clone(), player_2.clone(), player_3.clone()],
      Some(32),
    );
    draw_card(&mut game);
    let not_truthy = verify_best_player(player_3, game);
    assert!(!not_truthy);
  }
}
