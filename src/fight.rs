use crate::ai::AILevel;
use crate::morpion::PlayingState::Win;
use crate::morpion::{Morpion, Player, PlayingState};

/// Launches a series of AI vs AI fights.
/// Simulates `n` games between two AI levels and prints the results.
pub fn launch_fights(x_level: AILevel, o_level: AILevel, n: usize) {
    let mut f = n;
    let mut x_win = 0;
    let mut o_win = 0;
    let mut tie = 0;
    while f > 0 {
        println!("fight {} (X {:?} - O {:?}):", n - f + 1, x_level, o_level);
        let fight_result = fight(x_level, o_level);
        println!("{:?}", fight_result);
        match fight_result {
            Win(player) => {
                if player == Player::X {
                    x_win += 1;
                } else {
                    o_win += 1;
                }
            }
            _ => {
                tie += 1;
            }
        }
        f -= 1;
    }

    let total = n as f32;
    let x_stats = x_win as f32 / total * 100.0;
    let o_stats = o_win as f32 / total * 100.0;
    let tie_stats = tie as f32 / total * 100.0;
    println!(
        "-- fights results (total {}) -- \n=> X win ({:?}): {} ({}%)\n=> O win ({:?}): {} ({}%)\n=> tie: {} ({}%)",
        n, x_level, x_win, x_stats, o_level, o_win, o_stats, tie, tie_stats
    );
}

/// Simulates a single AI vs AI fight.
/// Plays a game of _Morpion_ between two AI players of specified levels and returns the game result.
fn fight(x_level: AILevel, o_level: AILevel) -> PlayingState {
    let mut morpion = Morpion::new();
    loop {
        morpion = match morpion.player {
            Player::X => morpion.ai_move(x_level),
            Player::O => morpion.ai_move(o_level),
        };
        if morpion.is_over() {
            break morpion.state;
        }
    }
}
