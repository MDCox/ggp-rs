use std::collections::HashMap;

use Player;

use gdl::{self, Goal, Rule, Role, Move, Score, GameDesc};
use self::MatchState::{Started, Playing, Finished};

#[derive(Eq, PartialEq)]
pub enum MatchState {
    Started, Playing, Finished
}

pub struct Game {
    match_state: MatchState,
    roles: Vec<Role>,
    role: Role,
    start_clock: u32,
    play_clock: u32,
    bases: Vec<Rule>,
    inputs: Vec<Rule>,
    legal: Vec<Rule>,
    next: Vec<Rule>,
    terminals: Vec<Rule>,
    goals: Vec<Goal>,
    init_state: State,
    cur_state: State
}

#[derive(Clone)]
pub struct State {
    props: HashMap<String, Rule>
}

impl State {
    pub fn new(props: HashMap<String, Rule>) -> State {
        State { props: props }
    }

    pub fn add_prop(rule: Rule) {

    }
}

impl Game {
    pub fn new(role: Role, start_clock: u32, play_clock: u32, roles: Vec<Role>, bases: Vec<Rule>,
           inputs: Vec<Rule>, legal: Vec<Rule>, next: Vec<Rule>, terminals: Vec<Rule>,
           goals: Vec<Goal>, init_state: State) -> Game {
        Game { match_state: Started, roles: roles, role: role, start_clock: start_clock,
               play_clock: play_clock, bases: bases, inputs: inputs, legal: legal, next: next,
               terminals: terminals, goals: goals, init_state: init_state.clone(),
               cur_state: init_state }
    }

    pub fn is_terminal(&self, state: State) -> bool {
        for rule in self.terminals.iter() {
            if rule.is_true(&state) {
                return true;
            }
        }
        false
    }

    pub fn get_roles(&self) -> &Vec<Role> {
        &self.roles
    }

    pub fn get_role(&self) -> &Role {
        &self.role
    }

    pub fn get_initial_state(&self) -> &State {
        &self.init_state
    }

    pub fn get_current_state(&self) -> &State {
        &self.cur_state
    }

    pub fn get_legal_moves(&self, state: &State, role: &Role) -> Vec<Move> {
        Vec::new() // TODO
    }

    pub fn get_goals(&self, state: &State) -> HashMap<Role, Score> {
        let mut m = HashMap::new();
        for goal in self.goals.iter() {
            if goal.rule.is_true(state) {
                assert!(m.insert(goal.role.clone(), goal.score).is_none());
            }
        }
        m
    }

    pub fn get_goal(&self, state: &State, role: &Role) -> Score {
        *self.get_goals(state).get(role).unwrap()
    }

    pub fn get_next_states(&self, state: &State) -> Vec<State> {
        panic!("unimplemented");
    }

    pub fn get_next_state(&self, state: &State, moves: Vec<Move>) -> State {
        let new_state = State::new();

        for rule in self.next.iter() {
            if rule.is_true(state) {
                new_state.add_prop(Base::new(rule.head));
            }
        }

        new_state
    }

    pub fn get_start_clock(&self) -> u32 {
        self.start_clock
    }

    pub fn get_play_clock(&self) -> u32 {
        self.play_clock
    }

    fn update(&mut self, moves: Vec<Move>) {
        if self.match_state != Playing {
            self.match_state = Playing;
        }
        self.cur_state = self.get_next_state(&self.cur_state, moves);
    }

    fn finish(&mut self, moves: Vec<Move>) {
        self.cur_state = self.get_next_state(&self.cur_state, moves);
        self.match_state = Finished;
    }
}

pub struct GameManager<P> {
    player: P,
    games: HashMap<String, Game>
}

impl<P: Player> GameManager<P> {
    pub fn new(p: P) -> GameManager<P> {
        GameManager { games: HashMap::new(), player: p }
    }

    pub fn handle(&mut self, request: String) -> String {
        // TODO: Merge these
        let start_re = regex!(r"\(start ([^ ]+) ([^ ]+) (\(.*\)) (\d+) (\d+)\)");
        if let Some(caps) = start_re.captures(&request) {
            assert_eq!(caps.len(), 6);
            return self.handle_start(caps.at(1).unwrap(), caps.at(2).unwrap(), caps.at(3).unwrap(),
                              caps.at(4).unwrap().parse().unwrap(),
                              caps.at(5).unwrap().parse().unwrap());
        }
        let play_re = regex!(r"\(play ([^ ]+) (\(.*\))\)");
        if let Some(caps) = play_re.captures(&request) {
            assert_eq!(caps.len(), 3);
            return self.handle_play(caps.at(1).unwrap(), caps.at(2).unwrap());
        }
        let stop_re = regex!(r"\(stop ([^ ]+) (\(.*\))\)");
        if let Some(caps) = stop_re.captures(&request) {
            assert_eq!(caps.len(), 3);
            return self.handle_stop(caps.at(1).unwrap(), caps.at(2).unwrap());
        }
        "".to_string()
    }

    fn handle_start(&mut self, match_id: &str, role: &str, game_desc: &str,
                    start_clock: u32, play_clock: u32) -> String {
        debug!("Handling start request");
        let desc = gdl::parse_desc(game_desc);
        let game = Game::new(Role::new(role), start_clock, play_clock, desc.roles, desc.bases,
                             desc.input, desc.legal, desc.next, desc.terminal, desc.goal,
                             desc.state);
        self.player.meta_game(&game);
        self.games.insert(match_id.to_string(), game);
        "ready".to_string()
    }

    fn handle_play(&mut self, match_id: &str, moves: &str) -> String {
        debug!("Handling play request");
        let game = self.games.get_mut(match_id).unwrap();
        game.update(Vec::new()); // TODO
        let m = self.player.select_move(game);
        m.to_string()
    }

    fn handle_stop(&mut self, match_id: &str, moves: &str) -> String {
        debug!("Handling stop request");
        let game = self.games.get_mut(match_id).unwrap();
        game.finish(Vec::new()); // TODO
        self.player.stop(game);
        "done".to_string()
    }
}
