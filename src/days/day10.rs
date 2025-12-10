use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    ops::{Deref, DerefMut},
};

use good_lp::{
    Expression, Solution, SolverModel, Variable, constraint, default_solver, variable, variables,
};
use winnow::{
    Parser, Result,
    ascii::{dec_uint, newline},
    combinator::{delimited, repeat, separated, seq},
    token::one_of,
};

use crate::days::Day;

/// A compact representation of the lights state for a machine
///
/// Each bit represents one light, with the LSB being the left-most light in the input.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lights(u16);

impl std::fmt::Debug for Lights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        write!(f, "{:b}", self.0)?;
        write!(f, "]")
    }
}

/// A compact representation of a button (which lights it toggles)
///
/// Each bit corresponds to one light, with the LSB being the left-most light in the input.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Button(u16);

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for b in 0..16 {
            let mask = 1 << b;
            if self.0 & mask > 0 {
                write!(f, "{b},")?;
            }
        }
        write!(f, ")")
    }
}

/// A machine definition, with the required end state for the lights and the available buttons + required joltages
#[derive(Debug, Clone, Hash)]
pub struct Machine {
    target: Lights,
    buttons: Vec<Button>,
    joltages: Vec<u16>,
}

impl Lights {
    /// Press a button (toggle some lights)
    fn press_button(self, button: Button) -> Self {
        Lights(*self ^ *button) // noice
    }
}

impl From<&[bool]> for Lights {
    /// Construct the compact representation from a list of booleans
    fn from(value: &[bool]) -> Self {
        let mut bits = 0;
        for (i, bit) in value.iter().enumerate() {
            if *bit {
                bits |= 1 << i;
            }
        }
        Self(bits)
    }
}

impl From<&[u8]> for Button {
    /// Construct the compact representation from a list of light positions
    fn from(value: &[u8]) -> Self {
        let mut bits = 0;
        for bit in value {
            bits |= 1 << bit;
        }
        Self(bits)
    }
}

/// Reconstruct the path (which buttons were pressed in which order) for A*
fn path(came_from: &HashMap<Lights, Button>, current: Lights) -> VecDeque<Button> {
    let mut path: VecDeque<Button> = VecDeque::new();
    let mut current = current;
    while came_from.contains_key(&current) {
        let button = *came_from.get(&current).unwrap();
        current = current.press_button(button);
        path.push_front(button);
    }
    path
}

/// Search the shortest sequence of button presses which leads to the target lights state with A*
fn a_star(buttons: &[Button], start: Lights, target: Lights) -> Option<usize> {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Candidate {
        cost: Reverse<u32>,
        state: Lights,
    }
    let mut open_set = BinaryHeap::<Candidate>::new(); // min-heap thanks to Reverse
    open_set.push(Candidate {
        cost: Reverse(1),
        state: start,
    });
    let mut came_from = HashMap::<Lights, Button>::new(); // find back the path taken
    let mut dist = HashMap::<Lights, u32>::new(); // travelled distance (number of buttons pressed)
    dist.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.state == target {
            let path = path(&came_from, current.state);
            return Some(path.len());
        }

        for button in buttons {
            let n = current.state.press_button(*button);
            // total number of button pressed if we pressed this button
            let tentative_dist = dist.get(&current.state).unwrap() + 1;
            if tentative_dist < *dist.get(&n).unwrap_or(&u32::MAX) {
                came_from.insert(n, *button);
                dist.insert(n, tentative_dist);
                // save neighbour as candidate
                // the heuristic must not overestimate the cost of reaching the target,
                // so we use the minimum possible number of button presses (1)
                let candidate = Candidate {
                    cost: Reverse(tentative_dist + 1),
                    state: n,
                };
                // replace this state in the min-heap
                open_set.retain(|c| c.state != candidate.state);
                open_set.push(candidate);
            }
        }
    }
    None
}

/// Retrieve the list of which buttons (index) increment a given joltage (identified by its position in the list)
fn buttons_idx_for_joltage(i: usize, buttons: &[Button]) -> Vec<usize> {
    let mask = 1u16 << i;
    buttons
        .iter()
        .enumerate()
        .filter_map(|(i, b)| (**b & mask > 0).then_some(i))
        .collect()
}

/// Identify the maximum number of possible button presses before we exceed any of the desired joltages
fn max_button_presses(mut button: Button, joltages: &[u16]) -> u16 {
    let mut min = u16::MAX;
    let mut b = 0;
    while *button > 0 {
        if button.trailing_ones() > 0 && joltages[b] < min {
            min = joltages[b];
        }
        *button >>= 1;
        b += 1;
    }
    min
}

pub struct Day10;

fn parse_lights(input: &mut &str) -> Result<Lights> {
    let lights: Vec<_> = delimited(
        '[',
        repeat(1.., one_of(('.', '#')).map(|c: char| c == '#')),
        ']',
    )
    .parse_next(input)?;
    Ok(lights.as_slice().into())
}

fn parse_button(input: &mut &str) -> Result<Button> {
    let indices: Vec<_> =
        delimited('(', separated(1.., dec_uint::<_, u8, _>, ','), ')').parse_next(input)?;
    Ok(indices.as_slice().into())
}

fn parse_buttons(input: &mut &str) -> Result<Vec<Button>> {
    separated(1.., parse_button, ' ').parse_next(input)
}

fn parse_joltages(input: &mut &str) -> Result<Vec<u16>> {
    delimited('{', separated(1.., dec_uint::<_, u16, _>, ','), '}').parse_next(input)
}

fn parse_machine(input: &mut &str) -> Result<Machine> {
    seq! {Machine {
        target: parse_lights,
        _: ' ',
        buttons: parse_buttons,
        _: ' ',
        joltages: parse_joltages
    }}
    .parse_next(input)
}

impl Day for Day10 {
    type Input = Vec<Machine>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        separated(1.., parse_machine, newline).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|machine| a_star(&machine.buttons, Lights::default(), machine.target).unwrap())
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut res = 0;
        for machine in input {
            // the problem to solve is a set of linear equations
            let mut problem = variables!();
            // the variables represent how many times we have to press each button
            let vars = machine.buttons.iter().map(|b| {
                variable()
                    .integer() // we're only interested in integer solutions
                    .min(0) // values can't be negative
                    .max(max_button_presses(*b, &machine.joltages)) // presses are bounded by the desired joltage
            });
            let vars: Vec<Variable> = problem.add_all(vars);
            // the objective to minimize is the sum of all button presses
            let objective: Expression = vars.iter().sum();
            let mut model = problem.minimise(objective).using(default_solver);
            // add constraints
            for (i, jolt) in machine.joltages.iter().copied().enumerate() {
                // for each joltage, first retrieve which buttons can affect it
                let buttons_idx = buttons_idx_for_joltage(i, &machine.buttons);
                // construct an expression which is the sum of all button presses for the buttons that can affect
                // this joltage
                let sum: Expression = buttons_idx.into_iter().map(|i| vars[i]).sum();
                // this is the target joltage
                let jolt = Expression::from_other_affine(jolt);
                // add a constraint that all button presses should equal to the joltage value
                model = model.with(constraint!(jolt == sum));
            }
            model.set_parameter("log", "0"); // disable logging
            let solution = model.solve().unwrap(); // magic 🪄
            // the sum of all variables is the total number of button presses, let's accumulate them
            res += vars
                .into_iter()
                .map(|v| solution.value(v) as usize)
                .sum::<usize>();
        }
        res
    }
}

impl Deref for Lights {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Lights {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Button {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
#[expect(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        let parsed = Day10::parser(&mut INPUT).unwrap();
        assert_eq!(Day10::part_1(&parsed), 7);
    }

    #[test]
    fn test_part2() {
        let parsed = Day10::parser(&mut INPUT).unwrap();
        assert_eq!(Day10::part_2(&parsed), 33);
    }
}
