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

struct Node {
    state: GameState,
    visits: u64,
    score: f64,
    children: Vec<(Column, Rc<RefCell<Node>>)>,
}

impl From<GameState> for Node {
    fn from(state: GameState) -> Self {
        Self {
            state,
            visits: 0,
            score: 0.0,
            children: vec![],
        }
    }
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn is_terminal(&self) -> bool {
        self.state.state.is_over()
    }

    fn computed_score(&self) -> f64 {
        if self.visits == 0 {
            f64::MAX
        } else {
            (self.score / self.visits as f64 + 1.0) / 2.0
        }
    }

    fn expand(&mut self) {
        let children = self
            .state
            .possible_moves()
            .iter()
            .map(|&c| {
                let mut s = self.state;
                s.apply_move(c);
                (c, Rc::new(RefCell::new(Node::from(s))))
            })
            .collect();
        self.children = children;
    }

    fn best_move(&self) -> (Column, Rc<RefCell<Node>>) {
        self.children
            .iter()
            .max_by_key(|(_, c)| {
                let c = c.borrow();
                c.visits
            })
            .unwrap()
            .clone()
    }

    fn best_child(&self) -> (Column, Rc<RefCell<Node>>) {
        self.children
            .iter()
            .max_by_key(|(_, c)| {
                let c = c.borrow();
                let score = c.score;
                let visits = c.visits as f64;
                if visits == 0.0 {
                    return OrderedF64(f64::MAX);
                }
                let parent_visits = self.visits as f64;
                let exploration = 1.8 * (parent_visits.ln() / visits).sqrt();
                let adjusted = (score / visits + 1.0) / 2.0;
                OrderedF64(adjusted + exploration)
            })
            .unwrap()
            .clone()
    }

    fn simulate<T>(&self, agent: &T) -> State
    where
        T: Agent,
    {
        let mut state = self.state;
        while !state.state.is_over() {
            let c = agent.next_move(&state);
            state.apply_move(c);
        }
        state.state
    }
}

struct SearchTree {
    root: Rc<RefCell<Node>>,
    random_agent: RandomAgent,
}

impl SearchTree {
    fn new(state: GameState) -> Self {
        let children = state
            .possible_moves()
            .iter()
            .map(|&c| {
                let mut s = state;
                s.apply_move(c);
                (c, Rc::new(RefCell::new(Node::from(s))))
            })
            .collect();
        let root = Rc::new(RefCell::new(Node {
            state,
            visits: 0,
            score: 0.0,
            children,
        }));
        SearchTree {
            root,
            random_agent: RandomAgent::default(),
        }
    }

    fn select(&self, node: &Rc<RefCell<Node>>) {
        let mut nodes = LinkedList::new();
        let current = {
            nodes.push_front(Rc::clone(node));
            let mut current = Rc::clone(node);
            while !current.borrow().is_leaf() {
                let (_, child) = current.borrow().best_child();
                {
                    let mut borrowed = current.borrow_mut();
                    borrowed.visits -= child.borrow().visits;
                    borrowed.score += child.borrow().score;
                }
                nodes.push_front(Rc::clone(&child));
                current = child;
            }
            if current.borrow().visits != 0 && !current.borrow().is_terminal() {
                current.borrow_mut().expand();
                let (_, child) = current.borrow().best_child();
                let mut borrowed = current.borrow_mut();
                borrowed.visits -= child.borrow().visits;
                borrowed.score += child.borrow().score;
                nodes.push_front(Rc::clone(&child));
            }
            nodes.pop_front().unwrap()
        };

        let state = current.borrow().state.state;
        match state {
            State::InProgress => {
                let score =
                    if let State::Win(player) = current.borrow().simulate(&self.random_agent) {
                        if player == current.borrow().state.current_player {
                            -1.0
                        } else {
                            1.0
                        }
                    } else {
                        0.0
                    };
                current.borrow_mut().score += score;
            }
            State::Draw => current.borrow_mut().score = 0.0,
            State::Win(player) => {
                let score = if player == current.borrow().state.current_player {
                    -1.0
                } else {
                    1.0
                } * current.borrow().visits as f64;
                current.borrow_mut().score = score;
            }
        }
        current.borrow_mut().visits += 1;

        nodes.into_iter().fold(Rc::clone(&current), |node, parent| {
            parent.borrow_mut().visits += node.borrow().visits;
            parent.borrow_mut().score -= node.borrow().score;
            parent
        });
    }
}

pub struct MctsAgent {
    iterations: usize,
    search_tree: RefCell<SearchTree>,
}

impl MctsAgent {
    pub fn new(iterations: usize, game_state: GameState) -> Self {
        Self {
            iterations,
            search_tree: RefCell::new(SearchTree::new(game_state)),
        }
    }
}

impl Agent for MctsAgent {
    fn next_move(&self, board: &GameState) -> Column {
        if *board != self.search_tree.borrow().root.borrow().state {
            let mut new_root = None;
            for (_, child) in self.search_tree.borrow().root.borrow().children.iter() {
                if child.borrow().state == *board {
                    new_root = Some(Rc::clone(child));
                    break;
                }
            }
            if let Some(new_root) = new_root {
                self.search_tree.borrow_mut().root = new_root;
            } else {
                println!("state not found. resetting search tree");
                *self.search_tree.borrow_mut() = SearchTree::new(*board);
            }
        }

        // let search_tree = SearchTree::new(*board);
        let (col, new_root) = {
            let search_tree = self.search_tree.borrow();
            for _ in 0..self.iterations {
                search_tree.select(&search_tree.root);
            }
            let borrowed = search_tree.root.borrow();
            let scores = borrowed
                .children
                .iter()
                .map(|(col, child)| {
                    let v = Rc::clone(child);
                    let child = child.borrow();
                    (col, child.visits, v, child.computed_score())
                })
                .collect::<Vec<_>>();
            for score in &scores {
                println!("{:?} - {:8} - {:3.5}", score.0, score.1, score.3 * 100.0);
            }
            let (col, new_root) = search_tree.root.borrow().best_move();
            (col, new_root)
        };

        self.search_tree.borrow_mut().root = new_root;
        col
    }
}
