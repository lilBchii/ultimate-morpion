use crate::ai::AILevel;
use crate::morpion::{Morpion, Player, PlayingState};
use crate::morpion::PlayingState::Win;

pub fn launch_fights(x_level: AILevel, o_level: AILevel, n: usize) {
    let mut f = n;
    let mut x_win = 0;
    let mut o_win = 0;
    let mut tie = 0;
    while f > 0 {
        println!("fight {} (X {:?} - O {:?}):", n-f+1, x_level, o_level);
        let fight_result = fight(x_level, o_level);
        println!("{:?}", fight_result);
        match fight_result {
            Win(player) => { if player == Player::X { x_win += 1; } else { o_win += 1; } },
            _ => { tie += 1; },
        }
        f -= 1;
    }

    let total = n as f32;
    let x_stats = x_win as f32/total*100.0;
    let o_stats = o_win as f32/total*100.0;
    let tie_stats = tie as f32/total*100.0;
    println!("-- fights results (total {}) -- \n=> X win: {x_win} ({}%)\n=> O win: {o_win} ({}%)\n=> tie: {tie} ({}%)",
             n, x_stats, o_stats, tie_stats);
}

fn fight(x_level: AILevel, o_level: AILevel) -> PlayingState {
    let mut morpion = Morpion::new();
    while !morpion.is_over() {
        //VERBOSE LOGS
        //println!("this is {:?}, state: {:?}", morpion.player, morpion.state);
        //println!("{}", morpion);
        morpion = match morpion.player {
            Player::X => { morpion.ai_move(x_level) },
            Player::O => { morpion.ai_move(o_level) },
        };
    }

    morpion.check_playing_state()
}