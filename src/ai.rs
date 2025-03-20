use crate::{CellState, Morpion, Player, PlayingState};
use rand::{self, Rng};

const WEIGHTS_CENTER: [isize; 9] = [40, 10, 40, 10, 45, 10, 40, 10, 40];
const WEIGHTS_CORNER: [isize; 9] = [45, 10, 45, 10, 15, 10, 45, 10, 45];
const WINNING_WEIGHT: isize = 10000;

/// Represents the different AI difficulty levels.
/// Determines the AI's decision-making complexity in the game.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AILevel {
    /// The easiest difficulty, making basic and predictable moves.
    Easy,
    /// A medium difficulty level with a better strategy.
    Medium,
    /// The hardest difficulty, utilizing advanced heuristics.
    Hard,
}

impl AILevel {
    /// Converts a string representation of AI difficulty level into an [`AILevel`] enum.
    /// Returns `None` if the input string does not match any known level.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "easy" => Some(AILevel::Easy),
            "medium" => Some(AILevel::Medium),
            "hard" => Some(AILevel::Hard),
            _ => None,
        }
    }
}

/// Implements the _Minimax algorithm_ for decision-making in the game.
/// Evaluates possible moves and returns the best score for the maximizing player.
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

/// Implements the _Alpha-Beta Pruning optimization_ for the _Minimax algorithm_.
/// Reduces the number of nodes evaluated by pruning branches that won't be selected.
pub fn alpha_beta(
    node: &Morpion,
    depth: isize,
    mut alpha: isize,
    mut beta: isize,
    maximizing_player: Player,
    heuristic: fn(&Morpion, Player) -> isize,
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

/// Determines the direction of evaluation for a given player.
/// Returns `1` if the actual player is the maximizing player, otherwise `-1`.
fn dir(actual_player: Player, maximizing_player: Player) -> isize {
    if actual_player == maximizing_player {
        1
    } else {
        -1
    }
}

/// Evaluates a game state using a weighted heuristic based on predefined weights.
/// Weights influence the importance of different positions on the board.
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

/// Heuristic function that prioritizes the center of the board.
/// Returns a score based on weighted positions with a preference for central control.
pub fn center_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    weighted_heuristic(node, maximizing_player, WEIGHTS_CENTER)
}

/// Heuristic function that prioritizes the corners of the board.
/// Returns a score based on weighted positions, favoring corner control.
pub fn corner_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    weighted_heuristic(node, maximizing_player, WEIGHTS_CORNER)
}

/// Evaluates the game state based on winning sequences.
/// Considers aligned marks that may lead to a win and assigns scores accordingly.
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

/// A comprehensive heuristic combining _winning sequences_ and _positional evaluation_.
/// Encourages strategic moves by considering both winning patterns and control zones.
pub fn everywhere_heuristic(node: &Morpion, maximizing_player: Player) -> isize {
    let mut score: isize = winning_sequence_heuristic(node, maximizing_player);
    if node.focused_big_cell.is_none() {
        score += dir(node.player, maximizing_player) * 2
    }

    score
}

/// Analyzes the board to find _winning sequences_.
/// A winning sequence is defined as two aligned marks in a row, column, or diagonal.
/// Returns a cumulative score for detected sequences.
/// The winning sequences can be cumulated.
/// ### Example
/// ```
/// X |  | X
/// ---------
/// O |  |
/// ---------
/// O |  |
/// ```
/// In this example, `X` has a winning sequence but not `O`.
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

/// Generates all possible game states from the current node by simulating valid moves.
/// Returns a vector of new game states representing all potential child nodes.
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

/// Generates a random noise value within the specified range.
/// Can be used to introduce randomness in AI decision-making.
pub fn noise(range: i32) -> isize {
    let mut rng = rand::rng();
    rng.random_range(-range..range) as isize
}
