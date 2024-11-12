#![allow(unused_mut, unused_variables, dead_code, unused_imports)]

use advent_of_code_2023_in_rust::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::{Debug, Formatter};

type ModuleName = &'static str;

const BUTTON: ModuleName = "button";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pulse {
    Low = 0,
    High,
}

struct PulseMessage {
    pulse: Pulse,
    from: ModuleName,
    to: ModuleName,
}

#[derive(Debug)]
struct Module {
    outputs: Vec<ModuleName>,
    module_type: ModuleType,
}

#[derive(Debug)]
enum ModuleType {
    Default,
    FlipFlop(Pulse),
    Conjunction(HashMap<ModuleName, Pulse>),
}

fn main() {
    // Parse the input
    //let input = include_str!("../../puzzle_inputs/day_20_test_00.txt");
    //let input = include_str!("../../puzzle_inputs/day_20_test_01.txt");
    let input = include_str!("../../puzzle_inputs/day_20.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    // Parse the input
    //let module_regex = Regex::new(r"([%&]?)(\w+) \-\> (\w|\,|\ )+").unwrap();
    let module_regex = Regex::new(r"([%&]?)(\w+) -> ((\w|\,|\ )+)").unwrap();
    let mut input: HashMap<ModuleName, Module> = parse_lines(module_regex, input)
        .map(|(module_type, module_name, outputs): (&str, &str, &str)| {
            assert!(
                module_name != BUTTON,
                "Cannot have an module named {}",
                BUTTON
            );
            let outputs: Vec<ModuleName> = outputs.split(", ").collect();
            let module_type = match module_type {
                "" => ModuleType::Default,
                "%" => ModuleType::FlipFlop(Pulse::Low),
                "&" => ModuleType::Conjunction(HashMap::new()),
                _ => panic!("Unknown module type: {}", module_type),
            };
            let module = Module {
                outputs,
                module_type,
            };
            (module_name, module)
        })
        .collect();

    // Initialize all the conjunction states
    let all_outputs: Vec<(ModuleName, Vec<ModuleName>)> = input
        .iter()
        .map(|(module_name, module)| (*module_name, module.outputs.clone()))
        .collect_vec();
    for (module_name, outputs) in all_outputs.iter() {
        for conjunction_name in outputs.iter() {
            if let Some(ModuleType::Conjunction(pulses)) =
                input.get_mut(conjunction_name).map(|m| &mut m.module_type)
            {
                pulses.insert(module_name, Pulse::Low);
            }
        }
    }

    // Let's run the simulation `n_sims` times
    let mut pulse_count = [0u64; 2];
    let n_sims = 1000;
    for sim in 0..n_sims {
        println!("\nSIM: {}\n", sim);
        let mut pulses: VecDeque<PulseMessage> = [PulseMessage {
            pulse: Pulse::Low,
            from: BUTTON,
            to: "broadcaster",
        }]
        .into();
        while let Some(pulse_message) = pulses.pop_front() {
            // debug - begin - print the puse message
            println!("{:?} ", pulse_message);
            // debug - end

            let PulseMessage { pulse, from, to } = pulse_message;

            // Add in the pulse count
            pulse_count[pulse as usize] += 1;

            if let Some(module) = input.get_mut(to) {
                let next_pulse = match &mut module.module_type {
                    ModuleType::Default => Some(pulse),
                    ModuleType::FlipFlop(state) => match pulse {
                        Pulse::Low => {
                            *state = state.flip();
                            Some(*state)
                        }
                        Pulse::High => None,
                    },
                    ModuleType::Conjunction(pulses) => {
                        pulses.insert(from, pulse);
                        match pulses.values().all(|&p| p == Pulse::High) {
                            true => Some(Pulse::Low),
                            false => Some(Pulse::High),
                        }
                    }
                };

                // Add the next pulse to the queue
                if let Some(next_pulse) = next_pulse {
                    for output in &module.outputs {
                        pulses.push_back(PulseMessage {
                            pulse: next_pulse,
                            from: to,
                            to: output,
                        });
                    }
                }
            }
        }
    }
    println!("\nFINISHED:");
    println!("pulse_count: {:?}", pulse_count);
    println!("answer: {}", pulse_count[0] * pulse_count[1]);
}

impl Pulse {
    fn flip(&self) -> Pulse {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

impl Debug for PulseMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pulse_str = match self.pulse {
            Pulse::Low => "low",
            Pulse::High => "high",
        };
        write!(f, "{} -{}-> {}", self.from, pulse_str, self.to)
    }
}

//// Solve 20a
//let sol_20a: usize = 12;
//let correct_sol_20a: usize = 32;
//println!("* 20a *");
//println!("My solution: {sol_20a}");
//println!("Correct solution: {correct_sol_20a}");
//println!("Equal: {:?}\n", sol_20a.cmp(&correct_sol_20a));
//
//// Solve 20b
//let sol_20b: usize = 56;
//let correct_sol_20b: usize = 79;
//println!("* 20b *");
//println!("My solution: {sol_20b}");
//println!("Correct solution: {correct_sol_20b}");
//println!("Equal: {:?}\n", sol_20b.cmp(&correct_sol_20b));
