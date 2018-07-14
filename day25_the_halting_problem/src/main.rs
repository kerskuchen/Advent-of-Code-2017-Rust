extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let (initial_state, num_steps, ruleset) = extract_rules(&input);
    let mut turing_machine = TuringMachine::new(0, initial_state, num_steps, ruleset);

    turing_machine.run();
    println!(
        "Checksum of the resulting tape: {}",
        turing_machine.checksum()
    );
}

struct Rule {
    zero_write: i32,
    zero_move: i32,
    zero_state: char,
    one_write: i32,
    one_move: i32,
    one_state: char,
}

struct TuringMachine {
    cur_pos: i32,
    cur_state: char,
    steps_left: usize,
    ruleset: HashMap<char, Rule>,
    tape: HashSet<i32>,
}

impl TuringMachine {
    fn new(
        initial_pos: i32,
        initial_state: char,
        num_steps: usize,
        ruleset: HashMap<char, Rule>,
    ) -> TuringMachine {
        TuringMachine {
            cur_pos: initial_pos,
            cur_state: initial_state,
            steps_left: num_steps,
            ruleset,
            tape: HashSet::new(),
        }
    }

    fn run(&mut self) {
        while self.steps_left != 0 {
            self.step()
        }
    }

    fn checksum(&self) -> usize {
        self.tape.len()
    }

    fn step(&mut self) {
        let rule = &self.ruleset[&self.cur_state];
        let (write_value, move_value, new_state) = if self.tape.get(&self.cur_pos).is_some() {
            // We have a one written at our current position
            (rule.one_write, rule.one_move, rule.one_state)
        } else {
            // We have a zero written at our current position
            (rule.zero_write, rule.zero_move, rule.zero_state)
        };

        if write_value == 0 {
            self.tape.remove(&self.cur_pos);
        } else {
            self.tape.insert(self.cur_pos);
        }

        self.cur_pos += move_value;
        self.cur_state = new_state;
        self.steps_left -= 1;
    }
}

/// Returns the initial state, the number of steps and the ruleset for the turing machine
fn extract_rules(input: &str) -> (char, usize, HashMap<char, Rule>) {
    let re_initial_values = Regex::new(
        r"Begin in state (?P<initial_state>.).
Perform a diagnostic checksum after (?P<num_steps>.*) steps.",
    ).unwrap()
        .captures_iter(input)
        .nth(0)
        .unwrap();

    let re_rules = Regex::new(
        r"In state (?P<state>.):
  If the current value is 0:
    - Write the value (?P<zero_write>.).
    - Move one slot to the (?P<zero_move>.*).
    - Continue with state (?P<zero_state>.).
  If the current value is 1:
    - Write the value (?P<one_write>.).
    - Move one slot to the (?P<one_move>.*).
    - Continue with state (?P<one_state>.).",
    ).unwrap();

    let initial_state: char = re_initial_values["initial_state"].chars().nth(0).unwrap();
    let num_steps: usize = re_initial_values["num_steps"].parse().unwrap();
    let ruleset: HashMap<char, Rule> = re_rules
        .captures_iter(input)
        .map(|rule| {
            let state = rule["state"].chars().nth(0).unwrap();
            let rule = Rule {
                zero_write: rule["zero_write"].parse().unwrap(),
                zero_move: match &rule["zero_move"] {
                    "left" => -1,
                    "right" => 1,
                    _ => panic!("Not a valid move"),
                },
                zero_state: rule["zero_state"].chars().nth(0).unwrap(),
                one_write: rule["one_write"].parse().unwrap(),
                one_move: match &rule["one_move"] {
                    "left" => -1,
                    "right" => 1,
                    _ => panic!("Not a valid move"),
                },
                one_state: rule["one_state"].chars().nth(0).unwrap(),
            };
            (state, rule)
        })
        .collect();

    (initial_state, num_steps, ruleset)
}
