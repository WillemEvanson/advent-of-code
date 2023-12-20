use std::collections::{HashMap, VecDeque};

use util::math::lcm;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut conjunctions = Vec::new();
    let mut modules = input
        .lines()
        .map(|str| {
            let (name, to) = str.split_once(" -> ").unwrap();
            let outputs = to.split(',').map(|str| str.trim()).collect::<Vec<_>>();
            if &name[..1] == "%" {
                (
                    &name[1..],
                    Module {
                        outputs,
                        kind: ModuleKind::FlipFlop(false),
                    },
                )
            } else if &name[..1] == "&" {
                conjunctions.push(&name[1..]);
                (
                    &name[1..],
                    Module {
                        outputs,
                        kind: ModuleKind::Conjunction(HashMap::new()),
                    },
                )
            } else if name == "broadcaster" {
                (
                    name,
                    Module {
                        outputs,
                        kind: ModuleKind::Broadcaster,
                    },
                )
            } else {
                panic!()
            }
        })
        .collect::<HashMap<_, _>>();

    for conjunction in conjunctions {
        let mut inputs = Vec::new();
        for (name, module) in modules.iter() {
            for &output in module.outputs.iter() {
                if output == conjunction {
                    inputs.push(name.to_owned());
                }
            }
        }
        let module = modules.get_mut(conjunction).unwrap();
        if let ModuleKind::Conjunction(map) = &mut module.kind {
            for input in inputs {
                map.insert(input, Pulse::Low);
            }
        } else {
            panic!();
        }
    }

    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(("broadcaster", Pulse::Low, "button"));

        while let Some((name, pulse, from)) = pulses.pop_front() {
            if let Pulse::Low = pulse {
                low += 1;
            } else {
                high += 1;
            }

            let module = if let Some(module) = modules.get_mut(name) {
                module
            } else {
                continue;
            };
            match &mut module.kind {
                ModuleKind::Conjunction(map) => {
                    *map.get_mut(from).unwrap() = pulse;
                    map.insert(from, pulse);
                    if map.values().any(|pulse| *pulse == Pulse::Low) {
                        for output in module.outputs.iter() {
                            pulses.push_back((output, Pulse::High, name));
                        }
                    } else {
                        for output in module.outputs.iter() {
                            pulses.push_back((output, Pulse::Low, name));
                        }
                    }
                }
                ModuleKind::FlipFlop(current) => {
                    if let Pulse::Low = pulse {
                        *current = !*current;
                        let pulse = if *current { Pulse::High } else { Pulse::Low };

                        for output in module.outputs.iter() {
                            pulses.push_back((output, pulse, name));
                        }
                    }
                }
                ModuleKind::Broadcaster => {
                    for output in module.outputs.iter() {
                        pulses.push_back((output, pulse, name));
                    }
                }
            }
        }
    }
    let part1 = low * high;

    let mut conjunctions = Vec::new();
    let mut modules = input
        .lines()
        .map(|str| {
            let (name, to) = str.split_once(" -> ").unwrap();
            let outputs = to.split(',').map(|str| str.trim()).collect::<Vec<_>>();
            if &name[..1] == "%" {
                (
                    &name[1..],
                    Module {
                        outputs,
                        kind: ModuleKind::FlipFlop(false),
                    },
                )
            } else if &name[..1] == "&" {
                conjunctions.push(&name[1..]);
                (
                    &name[1..],
                    Module {
                        outputs,
                        kind: ModuleKind::Conjunction(HashMap::new()),
                    },
                )
            } else if name == "broadcaster" {
                (
                    name,
                    Module {
                        outputs,
                        kind: ModuleKind::Broadcaster,
                    },
                )
            } else {
                panic!()
            }
        })
        .collect::<HashMap<_, _>>();

    for conjunction in conjunctions {
        let mut inputs = Vec::new();
        for (name, module) in modules.iter() {
            for &output in module.outputs.iter() {
                if output == conjunction {
                    inputs.push(name.to_owned());
                }
            }
        }
        let module = modules.get_mut(conjunction).unwrap();
        if let ModuleKind::Conjunction(map) = &mut module.kind {
            for input in inputs {
                map.insert(input, Pulse::Low);
            }
        } else {
            panic!();
        }
    }

    let mut first_to_reach = HashMap::new();
    let mut i = 0;
    let part2;
    'outer: loop {
        i += 1;
        let mut pulses = VecDeque::new();
        pulses.push_back(("broadcaster", Pulse::Low, "button"));

        while let Some((name, pulse, from)) = pulses.pop_front() {
            let module = if let Some(module) = modules.get_mut(name) {
                module
            } else {
                continue;
            };

            match &mut module.kind {
                ModuleKind::Conjunction(map) => {
                    if module.outputs[0] == "rx" && pulse == Pulse::High {
                        first_to_reach.entry(from).or_insert(i);
                        if first_to_reach.len() == map.len() {
                            part2 = first_to_reach
                                .values()
                                .fold(1, |accum, value| lcm(accum, *value));
                            break 'outer;
                        }
                    }

                    *map.get_mut(from).unwrap() = pulse;
                    map.insert(from, pulse);
                    if map.values().any(|pulse| *pulse == Pulse::Low) {
                        for output in module.outputs.iter() {
                            pulses.push_back((output, Pulse::High, name));
                        }
                    } else {
                        for output in module.outputs.iter() {
                            pulses.push_back((output, Pulse::Low, name));
                        }
                    }
                }
                ModuleKind::FlipFlop(current) => {
                    if let Pulse::Low = pulse {
                        *current = !*current;
                        let pulse = if *current { Pulse::High } else { Pulse::Low };

                        for output in module.outputs.iter() {
                            pulses.push_back((output, pulse, name));
                        }
                    }
                }
                ModuleKind::Broadcaster => {
                    for output in module.outputs.iter() {
                        pulses.push_back((output, pulse, name));
                    }
                }
            }
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Clone)]
struct Module<'a> {
    outputs: Vec<&'a str>,
    kind: ModuleKind<'a>,
}

#[derive(Debug, Clone)]
enum ModuleKind<'a> {
    Conjunction(HashMap<&'a str, Pulse>),
    FlipFlop(bool),
    Broadcaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}
