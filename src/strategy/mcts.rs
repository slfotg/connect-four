use crate::board::Column;
use crate::state::GameState;
use crate::state::State;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use super::random::RandomAgent;
use super::Agent;

#[derive(Default, Debug, Clone, Copy, PartialOrd, PartialEq)]
struct OrderedF64(f64);

impl Eq for OrderedF64 {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Score {
    score: f64,
    visits: u64,
}

impl Score {
    fn computed_score(&self) -> f64 {
        if self.visits == 0 {
            f64::MAX
        } else {
            self.score / self.visits as f64
        }
    }
}

struct Node {
    state: GameState,
    score: Score,
    children: Vec<(Column, Rc<RefCell<Node>>)>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn is_terminal(&self) -> bool {
        self.state.state.is_over()
    }

    fn expand(&mut self) {
        let children = self
            .state
            .possible_moves()
            .iter()
            .map(|&c| {
                let mut s = self.state;
                s.apply_move(c);
                (
                    c,
                    Rc::new(RefCell::new(Node {
                        state: s,
                        score: Score::default(),
                        children: vec![],
                    })),
                )
            })
            .collect();
        self.children = children;
    }

    fn best_child(&self) -> (Column, Rc<RefCell<Node>>) {
        self.children
            .iter()
            .max_by_key(|(_, c)| {
                let c = c.borrow();
                let score = c.score.score;
                let visits = c.score.visits as f64;
                if visits == 0.0 {
                    return OrderedF64(f64::MAX);
                }
                let parent_visits = self.score.visits as f64;
                let exploration = 2.0 * (parent_visits.ln() / visits).sqrt();
                OrderedF64((score / visits) + exploration)
            })
            .unwrap()
            .clone()
    }

    fn simulate(&self) -> State {
        let mut state = self.state;
        let agent = RandomAgent::default();
        while !state.state.is_over() {
            let c = agent.next_move(&state);
            state.apply_move(c);
        }
        state.state
    }
}

struct SearchTree {
    root: Rc<RefCell<Node>>,
}

impl SearchTree {
    fn new(state: GameState) -> Self {
        let children = state
            .possible_moves()
            .iter()
            .map(|&c| {
                let mut s = state;
                s.apply_move(c);
                (
                    c,
                    Rc::new(RefCell::new(Node {
                        state: s,
                        score: Score::default(),
                        children: vec![],
                    })),
                )
            })
            .collect();
        let root = Rc::new(RefCell::new(Node {
            state,
            score: Score::default(),
            children,
        }));
        SearchTree { root }
    }

    fn select(node: Rc<RefCell<Node>>) -> LinkedList<Rc<RefCell<Node>>> {
        let mut nodes = LinkedList::new();
        nodes.push_back(Rc::clone(&node));
        let mut current = Rc::clone(&node);
        while !current.borrow().is_leaf() {
            let (_, child) = current.borrow().best_child();
            nodes.push_back(Rc::clone(&child));
            current = child;
        }
        if current.borrow().score.visits != 0 && !current.borrow().is_terminal() {
            current.borrow_mut().expand();
            let (_, child) = current.borrow().best_child();
            nodes.push_back(Rc::clone(&child));
        }
        nodes
    }
}

pub struct MctsAgent {
    iterations: usize,
}

impl Default for MctsAgent {
    fn default() -> Self {
        Self {
            iterations: 500_000,
        }
    }
}

impl MctsAgent {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}

impl Agent for MctsAgent {
    fn next_move(&self, board: &GameState) -> Column {
        let search_tree = SearchTree::new(*board);
        for _ in 0..self.iterations {
            let nodes = SearchTree::select(search_tree.root.clone());
            let leaf = nodes.back().unwrap().clone();
            let winner = leaf.borrow().simulate();

            for node in nodes {
                let score = match winner {
                    State::Win(player) => {
                        if player == node.borrow().state.current_player {
                            0.0
                        } else {
                            1.0
                        }
                    }
                    State::Draw => 0.5,
                    State::InProgress => unreachable!(),
                };
                node.borrow_mut().score.score += score;
                node.borrow_mut().score.visits += 1;
            }
        }
        let borrowed = search_tree.root.borrow();
        let scores = borrowed
            .children
            .iter()
            .map(|(col, child)| {
                let child = child.borrow();
                let score = child.score;
                (col, score.visits, score.computed_score())
            })
            .collect::<Vec<_>>();
        println!("{:?}", scores);
        let col = search_tree.root.borrow().best_child().0;
        col
    }
}
