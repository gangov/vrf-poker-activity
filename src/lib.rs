use std::io::Read;
use rand::Rng;

use schnorrkel::{
  context::SigningContext,
  vrf::{VRFInOut, VRFProof, VRFProofBatchable},
  *,
};
use sp_core::blake2_256;

type DrawCardResult = (VRFInOut, VRFProof, VRFProofBatchable);

struct Poker {
  players: Vec<Player>,
  input: Option<u64>,
  signing_ctx: SigningContext,
}

impl Poker {
  fn new(players: Vec<Player>, input: Option<u64>) -> Self {
    Poker {
      players,
      input,
      signing_ctx: signing_context(b"Poker Game!")
    }
  }
}

#[derive(Debug, Clone)]
struct Player {
  pub random_number: Option<u32>,
  pub proof: Option<[u8; 32]>,
  pub drawed_card: Option<DrawCardResult>,
  pub key: Keypair,
}

impl Player {
  fn new() -> Self {
    let key = Keypair::generate();
    Self {
      random_number: None,
      proof: None,
      drawed_card: None,
      key,
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

fn generate_input(game: &mut Poker) {
  let mut rng = rand::thread_rng();

  // generate random number for all players & commit the hash for each random number
  for player in &mut game.players {
    let random_numer: u32 = rng.gen();
    player.random_number = Some(random_numer);
    player.proof = Some(blake2_256(&random_numer.to_le_bytes()));
  }


  // verify that all random number are correct
  // This is just to simulate the step
  // In reality, we would want to share the random number to each player
  // and have them verify the proof/hash themselves
  for player in &mut game.players {
    let calculated_hash = blake2_256(&player.random_number.unwrap().to_le_bytes());
    if !calculated_hash.eq(&player.proof.unwrap()) {
      panic!("Cheat detected!")
    }
  }

  let sum = game.players.iter().fold(0u64, |acc, x| acc + x.random_number.unwrap() as u64);

  game.input = Some(sum);
}

// 2. Take turn and draw card [x 5] for later
//   - VRF -> output % 52 = card number
//   - sign
fn draw_card(game: &mut Poker) {
  let input = game.input.expect("Input expected!");

  for mut one in &mut game.players {
    one.drawed_card = Some(one.key.vrf_sign(game.signing_ctx.bytes(&input.to_le_bytes())));
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

fn verify_best_player(player: Player, signing_context: &SigningContext, msg: &[u8]) -> bool {
  let signing_tx = signing_context.bytes(msg);
  let pre_out = player.drawed_card.as_ref().unwrap().0.to_preout();
  let out = &player.drawed_card.as_ref().unwrap().0;
  let proof = &player.drawed_card.as_ref().unwrap().1;
  let proof_batchable = &player.drawed_card.as_ref().unwrap().2;

  let (io, proof_result) = player
    .key
    .public
    .vrf_verify(signing_tx, &pre_out, &proof)
    .expect("failed to verify");

  if (&io == out) && (proof_batchable == &proof_result) {
    true
  } else {
    false
  }
}

#[cfg(test)]
mod test { use super::*;

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
  fn verify_input() {
    let players = generate_key_pairs(4);
    let mut game = Poker::new(players, None);
    generate_input(&mut game);

    assert!(game.input.is_some());
    println!("{:?}", game.input.unwrap());
  }

  #[test]
  fn play_poker() {
    let player = Keypair::generate();
    // 3. Take turn and commit the output [do it inside for loop]
    // 5. Find the winner from the output
  }
  #[test]
  fn test_find_best_player() {
    let mut players = generate_key_pairs(4);
    // draw_card(&mut playersßßΩΩs);
    let best_player = find_best_player(&players);
    assert!(best_player.drawed_card.is_some());
   }

  // write a test that will test the verify_best_player
  #[test]
  fn test_verify_best_player() {
    // let mut players = generate_key_pairs(4);
    // draw_card(&mut players);
    // let player = &players[0];
    // let signing_context = signing_context(b"Poker Game!");
    // let msg = b"I played";
    // let result = verify_best_player(player.clone(), &signing_context, msg);
    // assert_eq!(result, true);
  }
}
