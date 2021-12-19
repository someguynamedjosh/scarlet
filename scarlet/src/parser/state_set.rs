use std::{
    collections::HashSet,
    fmt::Debug,
    time::{Duration, Instant},
};

use indexmap::IndexSet;

use super::{
    rule::Rule,
    state::{ComponentMatch, State},
    token::Token,
};

#[derive(Clone, Debug, Default)]
pub struct PerfTimers {
    pub predict_time: Duration,
    pub scan_time: Duration,
    pub complete_time: Duration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateSet<'a> {
    pub position: usize,
    pub states: IndexSet<State<'a>>,
}

impl<'a> StateSet<'a> {
    pub fn new(rules: &'a [Rule], root_nonterminal: &str, timers: &mut PerfTimers) -> Self {
        let mut res = Self {
            position: 0,
            states: IndexSet::new(),
        };
        for rule in rules {
            if rule.produced_nonterminal == root_nonterminal {
                res.states.insert(State::new(rule, 0));
            }
        }
        let mut next_unprocessed = 0;
        loop {
            let old_size = res.states.len();
            res.predict(rules, 0, timers);
            res.complete(&[], next_unprocessed, timers);
            next_unprocessed = old_size;
            let new_size = res.states.len();
            if old_size == new_size {
                break;
            }
        }
        res
    }

    pub fn advance(
        rules: &'a [Rule],
        previous: &[Self],
        token: &Token<'a>,
        timers: &mut PerfTimers,
    ) -> Self {
        let immediate_predecessor = previous.last().unwrap();
        let mut next = Self {
            position: immediate_predecessor.position + 1,
            states: IndexSet::new(),
        };
        next.execute_steps_until_no_new_states_appear(rules, previous, token, timers);
        next
    }

    fn execute_steps_until_no_new_states_appear(
        &mut self,
        rules: &'a [Rule],
        previous: &[Self],
        token: &Token<'a>,
        timers: &mut PerfTimers,
    ) {
        let immediate_predecessor = previous.last().unwrap();
        let mut next_unprocessed = 0;
        loop {
            let old_size = self.states.len();
            self.predict(rules, 0, timers);
            self.scan(immediate_predecessor, token, timers);
            self.complete(previous, next_unprocessed, timers);
            next_unprocessed = old_size;
            let new_size = self.states.len();
            if old_size == new_size {
                break;
            }
        }
    }

    fn predict(&mut self, rules: &'a [Rule], after: usize, timers: &mut PerfTimers) {
        let start = Instant::now();

        let old_size = self.states.len();
        let mut new = HashSet::new();
        for existing in self.states.iter().skip(after) {
            if let Some(nt) = existing.immediate_next_nonterminal() {
                for rule in rules {
                    if rule.produced_nonterminal == nt {
                        new.insert(State::new(rule, self.position));
                    }
                }
            }
        }
        self.states.extend(new.into_iter());

        timers.predict_time += start.elapsed();
        if self.states.len() > old_size {
            self.predict(rules, old_size, timers);
        }
    }

    fn scan(&mut self, previous: &Self, token: &Token<'a>, timers: &mut PerfTimers) {
        let start = Instant::now();

        for state in &previous.states {
            if state.immediate_next_terminal_matches(token) {
                self.states.insert(state.advanced(ComponentMatch::ByToken));
            }
        }

        timers.scan_time += start.elapsed();
    }

    fn get_state_completing_nonterminal(&self, nonterminal: &str) -> Option<usize> {
        let mut backup = None;
        for (completed_state_index, state) in self.states.iter().enumerate() {
            if !state.is_complete() {
                continue;
            }
            if state.rule.produced_nonterminal == nonterminal {
                if state.rule.preferred {
                    return Some(completed_state_index);
                } else {
                    backup = Some(completed_state_index);
                }
            }
        }
        backup
    }

    fn complete_state(
        state: &State,
        index: usize,
        existing_states: &IndexSet<State<'a>>,
        previous: &[Self],
        new: &mut HashSet<State<'a>>,
    ) {
        let idx = state.start_position_in_input;
        let previous_states = if idx < previous.len() {
            &previous[idx].states
        } else {
            existing_states
        };
        for previous_state in previous_states {
            if previous_state.immediate_next_nonterminal() == Some(&state.rule.produced_nonterminal)
            {
                let mat = ComponentMatch::ByState(index);
                new.insert(previous_state.advanced(mat));
            }
        }
    }

    fn complete(&mut self, previous: &[Self], after: usize, timers: &mut PerfTimers) {
        let start = Instant::now();

        let mut completed_nonterminals = HashSet::new();
        for state in self.states.iter() {
            if state.is_complete() {
                completed_nonterminals.insert(&state.rule.produced_nonterminal);
            }
        }
        let mut new = HashSet::new();
        for completed_nonterminal in completed_nonterminals {
            let completed_state_index = self
                .get_state_completing_nonterminal(completed_nonterminal)
                .unwrap();
            let state = &self.states[completed_state_index];
            Self::complete_state(
                state,
                completed_state_index,
                &self.states,
                previous,
                &mut new,
            )
        }
        for (index, state) in self.states.iter().enumerate() {
            if !state.is_complete() {
                continue;
            }
            Self::complete_state(state, index, &self.states, previous, &mut new)
        }
        self.states.extend(new.into_iter());

        timers.complete_time += start.elapsed();
    }
}
