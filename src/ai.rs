use crate::{CellState, Morpion, Player, PlayingState};
use rand::{self, Rng};

const WEIGHTS_CENTER: [isize; 9] = [40, 10, 40, 10, 45, 10, 40, 10, 40];
const WEIGHTS_CORNER: [isize; 9] = [45, 10, 45, 10, 15, 10, 45, 10, 45];
const WINNING_WEIGHT: isize = 10000;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AILevel {
    Easy,
    Medium,
    Hard,
}

pub fn minimax(
    node: &Morpion,
    depth: isize,
    maximizing_player: Player,
    heuristic: &dyn Fn(&Morpion, Player) -> isize,
) -> isize {
    if node.state != PlayingState::Continue || depth == 0 {
        return heuristic(node, maximizing_player);
    }
    if node.player == maximizing_player {
        let mut value = isize::MIN;
        for child in generate_children(node) {
            value = value.max(minimax(&child, depth - 1, maximizing_player, heuristic));
        }
        return value;
    }
    let mut value = isize::MAX;
    for child in generate_children(node) {
        value = value.min(minimax(&child, depth - 1, maximizing_player, heuristic));
    }
    value
}

pub fn alpha_beta(
    node: &Morpion,
    depth: isize,
    mut alpha: isize,
    mut beta: isize,
    maximizing_player: Player,
    heuristic: &dyn Fn(&Morpion, Player) -> isize,
) -> isize {
    if node.state != PlayingState::Continue || depth == 0 {
        return heuristic(node, maximizing_player) * (depth + 1);
    }
    if node.player == maximizing_player {
        let mut value = isize::MIN;
        for child in generate_children(node) {
            value = value.max(alpha_beta(
                &child,
                depth - 1,
                alpha,
                beta,
                maximizing_player,
                heuristic,
            ));
            if value > beta {
                break;
            }
            alpha = alpha.max(value);
        }
        return value;
    }
    let mut value = isize::MAX;
    for child in generate_children(node) {
        value = value.min(alpha_beta(
            &child,
            depth - 1,
            alpha,
            beta,
            maximizing_player,
            heuristic,
        ));
        if value < alpha {
            break;
        }
        beta = beta.min(value);
    }
    value
}

fn dir(actual_player: Player, maximizing_player: Player) -> isize {
    if actual_player == maximizing_player {
        1
    } else {
        -1
    }
}

fn weighted_heuristic(node: &Morpion, maximizing_player: Player, weights: [isize; 9]) -> isize {
    let mut score: isize = 0;
    match node.state {
        PlayingState::Continue => {
            for big_cell_index in 0..9 {
                match node.board.states[big_cell_index] {
                    CellState::Occupied(player) => {
                        score += dir(player, maximizing_player) * 50 * weights[big_cell_index]
                    }
                    CellState::Tie => {}
                    CellState::Free => {
                        for lil_cell_index in 0..9 {
                            if let CellState::Occupied(player) =
                                node.board.cells[big_cell_index][lil_cell_index]
                            {
                                score += dir(player, maximizing_player) * weights[lil_cell_index];
                            }
                        }
                    }
                }
            }
        }
        PlayingState::Win(player) => score += dir(player, maximizing_player) * WINNING_WEIGHT,
        PlayingState::Tie => {}
    }

    score
}

pub fn center_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    weighted_heuristic(node, maximizing_player, WEIGHTS_CENTER)
}

pub fn corner_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    weighted_heuristic(node, maximizing_player, WEIGHTS_CORNER)
}

pub fn winning_sequence_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    let mut score: isize = 0;
    match node.state {
        PlayingState::Continue => {
            score += evaluate_winning_sequence(&node.board.states, maximizing_player) * 2;
            for big_cell_index in 0..9 {
                match node.board.states[big_cell_index] {
                    CellState::Occupied(player) => {
                        let dir = dir(player, maximizing_player);
                        score += dir * 5;
                        if big_cell_index == 4 {
                            score += dir * 10;
                        } else if big_cell_index == 0
                            || big_cell_index == 2
                            || big_cell_index == 6
                            || big_cell_index == 8
                        {
                            score += dir * 3;
                        }
                    }
                    CellState::Free => {
                        score += evaluate_winning_sequence(
                            &node.board.cells[big_cell_index],
                            maximizing_player,
                        );
                        for lil_cell_index in 0..9 {
                            if let CellState::Occupied(player) =
                                node.board.cells[big_cell_index][lil_cell_index]
                            {
                                let dir = dir(player, maximizing_player);
                                if lil_cell_index == 4 {
                                    score += dir * 3;
                                }
                                if big_cell_index == 4 {
                                    score += dir * 3;
                                }
                            }
                        }
                    }
                    CellState::Tie => {}
                }
            }
        }
        PlayingState::Win(player) => score += dir(player, maximizing_player) * WINNING_WEIGHT,
        PlayingState::Tie => {}
    }

    score
}

pub fn everywhere_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    let mut score: isize = winning_sequence_heuristic(node, maximizing_player);
    if node.focused_big_cell.is_none() {
        score += dir(node.player, maximizing_player) * 2
    }

    score
}

// Travel a morpion grid searching for a winning sequence.
// A winning sequence is when a player has marked two aligned cells by row, column or diagonal:
// X |  | X
// ---------
// O |  |
// ---------
// O |  |
// In this example, X has a winning sequence but not O.
// The function attributes a score of 2 for each winning sequence. The winning sequences are cumulated.
pub fn evaluate_winning_sequence(states: &[CellState; 9], maximizing_player: Player) -> isize {
    let mut score: isize = 0;
    let mut diag1_score: isize = 0;
    let mut diag2_score: isize = 0;
    for row in 0..3 {
        let mut row_score = 0;
        let mut col_score = 0;
        for col in 0..3 {
            if let CellState::Occupied(player) = states[row * 3 + col] {
                row_score += dir(player, maximizing_player);
            }
            if let CellState::Occupied(player) = states[col * 3 + row] {
                col_score += dir(player, maximizing_player);
            }
            if row + col == 2 {
                if let CellState::Occupied(player) = states[row * 3 + col] {
                    diag2_score += dir(player, maximizing_player);
                }
            }
        }
        if let CellState::Occupied(player) = states[row * 4] {
            diag1_score += dir(player, maximizing_player);
        }
        if row_score % 2 == 0 {
            score += row_score;
        }
        if col_score % 2 == 0 {
            score += col_score;
        }
    }
    if diag1_score % 2 == 0 {
        score += diag1_score;
    }
    if diag2_score % 2 == 0 {
        score += diag2_score;
    }
    score
}

pub fn generate_children(node: &Morpion) -> Vec<Morpion> {
    let mut children = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            if node.index_is_playable(i, j) {
                let mut new_node = node.clone();
                new_node.play_at(i, j);
                children.push(new_node);
            }
        }
    }
    children
}

pub fn noise(range: i32) -> isize {
    let mut rng = rand::rng();
    rng.random_range(-range..range) as isize
}
