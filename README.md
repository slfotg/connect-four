# Connect Four

A Connect Four bot in Rust

## Quickstart
```
cargo build --release
./target/release/main
```

## TODO:
- [ ] Change the Score type in mcts to be an enum with known Win/Lose/Draw variants that keep track of how many moves until the game should end.

    When the bot is in a losing position, it can just play random moves when it prevent an immediate loss.

    A losing bot should try to extend its life as long as possible and a winning bot should try to win as fast as possible.

    Score should implement Ord based on its variants.

- [ ] Propagate Win/Lose variants to the parent.

    For example, if a single child is a win then the parent is in a losing position.

    Similarly, if all children are in a losing position then the parent is winning.

    This should prune search branches faster than any other kind of optimization or technique. A lot of games are in a state that is obviously known by both players but can continue for several moves.

- [ ] When the root of the search tree is in a known end state, searching should stop and the moves given by the bot should try to extend the game as long as possible for losing and drawn positions and try to end the game as fast as possible for winning positions.