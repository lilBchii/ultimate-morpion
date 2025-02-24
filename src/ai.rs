use crate::{CellState, GameState, Morpion, Player};

pub fn alpha_beta(
    node: &Morpion,
    depth: isize,
    mut alpha: isize,
    mut beta: isize,
    maximizing_player: bool,
) -> isize {
    if node.state != GameState::Continue || depth == 0 {
        return first_heuristic(node);
    }
    if maximizing_player {
        let mut value = isize::MIN;
        for child in generate_children(node) {
            value = value.max(alpha_beta(&child, depth - 1, alpha, beta, false));
            if value > beta {
                break;
            }
            alpha = alpha.max(value);
        }
        return value;
    }
    let mut value = isize::MAX;
    for child in generate_children(node) {
        value = value.min(alpha_beta(&child, depth - 1, alpha, beta, true));
        if value < alpha {
            break;
        }
        beta = beta.min(value);
    }
    value
}

pub fn first_heuristic(node: &Morpion) -> isize {
    let weights: [isize; 9] = [40, 10, 40, 10, 45, 10, 40, 10, 40];
    let mut score: isize = 0;
    match node.state {
        GameState::Continue => {
            for big_cell_index in 0..9 {
                match node.board.states[big_cell_index] {
                    CellState::Occupied(player) => match player {
                        Player::X => score += -50 * weights[big_cell_index],
                        Player::O => score += 50 * weights[big_cell_index],
                    },
                    CellState::Tie => {}
                    CellState::Free => {
                        for lil_cell_index in 0..9 {
                            if let CellState::Occupied(player) =
                                node.board.cells[big_cell_index][lil_cell_index]
                            {
                                match player {
                                    Player::X => score += -weights[lil_cell_index],
                                    Player::O => score += weights[lil_cell_index],
                                }
                            }
                        }
                    }
                }
            }
        }
        GameState::Win(player) => match player {
            Player::X => score = -5000,
            Player::O => score = 5000,
        },
        GameState::Tie => score = 0,
    }
    score
}

pub fn generate_children(node: &Morpion) -> Vec<Morpion> {
    let mut child = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            if node.index_is_playable(i, j) {
                let mut new_node = node.clone();
                new_node.play_at(i, j);
                child.push(new_node);
            }
        }
    }
    child
}
