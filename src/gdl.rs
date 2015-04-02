use game_manager::State;

use gdl_parser::{self, parse};
use gdl_parser::SExpr::{self, Func, RuleOp};
use gdl_parser::Term::{Constant, ExprTerm, Num};

#[derive(Clone)]
pub struct Rule {
    head: Sentence,
    body: Vec<Literal>
}

impl Rule {
    pub fn is_true(&self, state: &State) -> bool {
        true // TODO
    }
}

pub struct Goal {
    pub role: Role,
    pub rule: Rule,
    pub score: Score
}

#[derive(Clone)]
enum Sentence {
    PropSentence(Proposition),
    RelSentence(Relation)
}

#[derive(Clone)]
enum Literal {
    NotLit(Not),
    OrLit(Or),
    DistinctLit(Distinct),
    PropLit(Proposition),
    RelLit(Relation)
}

#[derive(Clone)]
enum Term {
    VarTerm(Variable),
    FuncTerm(Function),
    ConstTerm(Constant)
}

#[derive(Clone)]
struct Proposition {
    head: Constant
}

#[derive(Clone)]
struct Relation {
    name: Constant,
    args: Vec<Term>
}

#[derive(Clone)]
struct Not {
    lit: Box<Literal>
}

#[derive(Clone)]
struct Or {
    lits: Vec<Literal>
}

#[derive(Clone)]
struct Distinct {
    term1: Term,
    term2: Term
}

#[derive(Clone)]
struct Variable {
    name: String
}

#[derive(Clone)]
struct Function {
    name: Constant,
    args: Vec<Term>
}

#[derive(Clone)]
struct Constant {
    name: String
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Role {
    pub name: String
}

impl Role {
    pub fn new(name: &str) -> Role {
        Role { name: name.to_string() }
    }
}

pub struct Move;

impl ToString for Move {
    fn to_string(&self) -> String {
        "".to_string() // TODO
    }
}

pub type Score = u8;

pub struct GameDesc {
    pub roles: Vec<Role>,
    pub bases: Vec<Rule>,
    pub input: Vec<Rule>,
    pub legal: Vec<Rule>,
    pub next: Vec<Rule>,
    pub terminal: Vec<Rule>,
    pub goal: Vec<Goal>,
    pub state: State
}

impl GameDesc {
    fn new() -> GameDesc {
        GameDesc { roles: Vec::new(), bases: Vec::new(), input: Vec::new(), legal: Vec::new(),
                   next: Vec::new(), terminal: Vec::new(), goal: Vec::new(),
                   state: State::new(Vec::new()) }
    }
}

pub fn parse_desc(gdl: &str) -> GameDesc {
    let mut desc = GameDesc::new();
    // let gdl = parse(gdl);
    // for expr in gdl.sexprs {
    //     visit_sexpr(expr, &mut desc);
    // }
    desc
}

// fn visit_sexpr(expr: SExpr, desc: &mut GameDesc) {
//     match expr {
//         Func(name, terms) => visit_func(name, terms, desc),
//         RuleOp(terms) => visit_rule(terms, desc)
//     };
// }

// fn visit_func(name: String, terms: Vec<gdl_parser::Term>, desc: &mut GameDesc) {
//     match &name {
//         "role" => (),
//         "base" => (),
//         "init" => (),
//     };
// }

// fn visit_rule(terms: Vec<gdl_parser::Term>, desc: &mut GameDesc) {

// }
