use crate::{CellState, Morpion, Player, PlayingState};

const WEIGHTS: [isize; 9] = [40, 10, 40, 10, 45, 10, 40, 10, 40];

pub fn minimax(node: &Morpion, depth: isize, maximizing_player: Player) -> isize {
    if node.state != PlayingState::Continue || depth == 0 {
        return first_heuristic(node, maximizing_player);
    }
    if node.player == maximizing_player {
        let mut value = isize::MIN;
        for child in generate_children(node) {
            value = value.max(minimax(&child, depth - 1, maximizing_player));
        }
        return value;
    }
    let mut value = isize::MAX;
    for child in generate_children(node) {
        value = value.min(minimax(&child, depth - 1, maximizing_player));
    }
    value
}

pub fn alpha_beta(
    node: &Morpion,
    depth: isize,
    mut alpha: isize,
    mut beta: isize,
    maximizing_player: Player,
) -> isize {
    if node.state != PlayingState::Continue || depth == 0 {
        return first_heuristic(node, maximizing_player);
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
        PlayingState::Tie => score = 0,
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
