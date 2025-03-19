use connect_four::board::Player;
use connect_four::display::term::BoardAnsiWriter;
use connect_four::state::GameState;
use connect_four::strategy::cli::CliAgent;
use connect_four::strategy::mcts;
use connect_four::strategy::mcts2;
use connect_four::strategy::Agent;

fn main() {
    let mut board = GameState::default();
    println!("{}", BoardAnsiWriter(board.board));
    let mut player = Player::Yellow;

    let player1 = mcts::MctsAgent::new(100_000, board);
    let player2 = mcts2::MctsAgent::new(100_000, board);

    while !board.state.is_over() {
        let c = if player == Player::Yellow {
            player1.next_move(&board)
        } else {
            player2.next_move(&board)
        };
        board.apply_move(c);
        println!("{}", BoardAnsiWriter(board.board));
        player = !player;
    }
    println!("{:?}", board.state);
}
