use crate::{CellState, Morpion, Player, PlayingState};

const WEIGHTS: [isize; 9] = [40, 10, 40, 10, 45, 10, 40, 10, 40];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AILevel {
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for AILevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Easy => "Easy",
                Self::Medium => "Medium",
                Self::Hard => "Hard",
            }
        )
    }
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
        return heuristic(node, maximizing_player)*(depth+1);
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

pub fn first_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    let dir = |player: Player| if player == maximizing_player { 1 } else { -1 };
    let mut score: isize = 0;
    match node.state {
        PlayingState::Continue => {
            for big_cell_index in 0..9 {
                match node.board.states[big_cell_index] {
                    CellState::Occupied(player) => {
                        score += dir(player) * 50 * WEIGHTS[big_cell_index]
                    }
                    CellState::Tie => {}
                    CellState::Free => {
                        for lil_cell_index in 0..9 {
                            if let CellState::Occupied(player) =
                                node.board.cells[big_cell_index][lil_cell_index]
                            {
                                score += dir(player) * WEIGHTS[lil_cell_index];
                            }
                        }
                    }
                }
            }
        }
        PlayingState::Win(player) => score += dir(player) * 5000,
        PlayingState::Tie => {}
    }

    score
}

pub fn second_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    let dir = |player: Player| if player == maximizing_player { 1 } else { -1 };
    let mut score: isize = 0;
    match node.state {
        PlayingState::Continue => {
            score += evaluate_winning_sequence(&node.board.states, maximizing_player);
            for big_cell_index in 0..9 {
                match node.board.states[big_cell_index] {
                    CellState::Occupied(player) => {
                        score += dir(player) * 5;
                        if big_cell_index == 4 {
                            score += dir(player) * 10;
                        } else if big_cell_index == 0
                            || big_cell_index == 2
                            || big_cell_index == 6
                            || big_cell_index == 8
                        {
                            score += dir(player) * 3;
                        }
                    }
                    CellState::Free => {
                        score += evaluate_winning_sequence(
                            &node.board.cells[big_cell_index],
                            maximizing_player,
                        ) / 2;
                        for lil_cell_index in 0..9 {
                            if let CellState::Occupied(player) =
                                node.board.cells[big_cell_index][lil_cell_index]
                            {
                                if lil_cell_index == 4 {
                                    score += dir(player) * 3;
                                }
                                if big_cell_index == 4 {
                                    score += dir(player) * 3;
                                }
                            }
                        }
                    }
                    CellState::Tie => {}
                }
            }
        }
        PlayingState::Win(player) => score += dir(player) * 10000,
        PlayingState::Tie => {}
    }

    score
}

pub fn evaluate_winning_sequence(states: &[CellState; 9], maximizing_player: Player) -> isize {
    let dir = |player: Player| if player == maximizing_player { 1 } else { -1 };
    let mut score: isize = 0;
    let mut diag1_score: isize = 0;
    let mut diag2_score: isize = 0;
    for row in 0..3 {
        let mut row_score = 0;
        let mut col_score = 0;
        for col in 0..3 {
            if let CellState::Occupied(player) = states[row * 3 + col] {
                row_score += dir(player);
            }
            if let CellState::Occupied(player) = states[col * 3 + row] {
                col_score += dir(player);
            }
            if row + col == 2 {
                if let CellState::Occupied(player) = states[row * 3 + col] {
                    diag2_score += dir(player);
                }
            }
        }
        if let CellState::Occupied(player) = states[row * 4] {
            diag1_score += dir(player);
        }
        if row_score % 2 == 0 {
            score += row_score * 2;
        }
        if col_score % 2 == 0 {
            score += col_score * 2;
        }
    }
    if diag1_score % 2 == 0 {
        score += diag1_score * 2;
    }
    if diag2_score % 2 == 0 {
        score += diag2_score * 2;
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
