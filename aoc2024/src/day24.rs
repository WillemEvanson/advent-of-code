use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (registers, comp) = input.split_once("\n\n").unwrap();

    let mut graph = Vec::new();
    let mut names = Vec::new();
    let mut map = HashMap::new();
    for line in registers.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        let value = match value {
            "0" => false,
            "1" => true,
            _ => panic!(),
        };

        let key = if let Some(&i) = map.get(name) {
            i
        } else {
            let i = graph.len() as u32;
            graph.push(Input::Initial(false));
            names.push(name);
            map.insert(name, i);
            i
        };

        graph[key as usize] = Input::Initial(value);
    }

    for line in comp.lines() {
        let (computation, result) = line.split_once(" -> ").unwrap();
        let (input0, remaining) = computation.split_once(' ').unwrap();
        let (gate, input1) = remaining.split_once(' ').unwrap();

        let gate = match gate {
            "XOR" => Gate::Xor,
            "AND" => Gate::And,
            "OR" => Gate::Or,
            _ => panic!(),
        };

        let result = if let Some(&i) = map.get(result) {
            i
        } else {
            let i = graph.len() as u32;
            graph.push(Input::Initial(false));
            names.push(result);
            map.insert(result, i);
            i
        };

        let input0 = if let Some(&i) = map.get(input0) {
            i
        } else {
            let i = graph.len() as u32;
            graph.push(Input::Initial(false));
            names.push(input0);
            map.insert(input0, i);
            i
        };

        let input1 = if let Some(&i) = map.get(input1) {
            i
        } else {
            let i = graph.len() as u32;
            graph.push(Input::Initial(false));
            names.push(input1);
            map.insert(input1, i);
            i
        };

        graph[result as usize] = Input::Gate(gate, input0, input1);
    }

    let mut i = 0;
    let mut part1 = 0;
    while let Some(&idx) = map.get(format!("z{i:02}").as_str()) {
        let val = find_recursive(idx, &graph);
        part1 += (val as u64) << i;
        i += 1;
    }

    let mut i = 0;
    let mut x_inputs = Vec::new();
    while let Some(&idx) = map.get(format!("x{i:02}").as_str()) {
        x_inputs.push(idx);
        i += 1;
    }

    let mut i = 0;
    let mut y_inputs = Vec::new();
    while let Some(&idx) = map.get(format!("y{i:02}").as_str()) {
        y_inputs.push(idx);
        i += 1;
    }

    let mut i = 0;
    let mut outputs = Vec::new();
    while let Some(&idx) = map.get(format!("z{i:02}").as_str()) {
        outputs.push(idx);
        i += 1;
    }

    let mut found = Vec::new();
    for &i in outputs.iter().take(outputs.len() - 1) {
        // Check that every 'z' only comes from a XOR (this must be true for every
        // output except for z45
        match graph[i as usize] {
            Input::Gate(Gate::Xor, _, _) => (),
            _ => found.push(i),
        }
    }

    let mut input_gates = Vec::new();
    for i in 0..graph.len() as u32 {
        // Check that every XOR that takes in 'x__' and 'y__' doesn't output 'z__',
        // unless it is z00, since that is the base case.
        if let Input::Gate(Gate::Xor, input0, input1) = graph[i as usize] {
            if i == outputs[0] {
                continue;
            }

            let takes_input = (x_inputs.contains(&input0) && y_inputs.contains(&input1))
                || (x_inputs.contains(&input1) && y_inputs.contains(&input0));
            if takes_input {
                input_gates.push(i);
            }

            let gives_output = outputs.contains(&i);

            if (gives_output && takes_input) || (!gives_output && !takes_input) {
                found.push(i);
            }
        }
    }

    let mut and_inputs = Vec::new();
    for i in 0..graph.len() as u32 {
        // Check that every XOR feeds into either a XOR or another AND.
        if let Input::Gate(gate, input0, input1) = graph[i as usize] {
            if !matches!(gate, Gate::Xor | Gate::And) {
                if input_gates.contains(&input0) {
                    found.push(input0);
                }

                if input_gates.contains(&input1) {
                    found.push(input1);
                }
            }

            if matches!(gate, Gate::And) {
                and_inputs.push(i);
            }
        }
    }

    for i in 0..graph.len() as u32 {
        // Check that every AND that takes in 'x__' and 'y__' feeds into an AND or OR.
        if let Input::Gate(gate, input0, input1) = graph[i as usize] {
            if !matches!(gate, Gate::And | Gate::Or) {
                if i == outputs[1] {
                    continue;
                }

                if and_inputs.contains(&input0) {
                    found.push(input0);
                }

                if and_inputs.contains(&input1) {
                    found.push(input1);
                }
            }
        }
    }

    let mut found_names = found.iter().map(|&i| names[i as usize]).collect::<Vec<_>>();
    found_names.sort_unstable();
    found_names.dedup();

    let mut part2 = String::new();
    part2.push_str(found_names[0]);
    for name in found_names.iter().skip(1) {
        part2.push(',');
        part2.push_str(name);
    }

    Solution::from((part1, part2))
}

fn find_recursive(current: u32, gates: &[Input]) -> bool {
    match gates[current as usize] {
        Input::Initial(val) => val,
        Input::Gate(gate, input0, input1) => {
            let input0 = find_recursive(input0, gates);
            let input1 = find_recursive(input1, gates);
            gate.simulate(input0, input1)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Input {
    Initial(bool),
    Gate(Gate, u32, u32),
}

#[derive(Debug, Clone, Copy)]
enum Gate {
    And,
    Xor,
    Or,
}

impl Gate {
    fn simulate(self, input0: bool, input1: bool) -> bool {
        match self {
            Self::And => input0 && input1,
            Self::Or => input0 || input1,
            Self::Xor => (input0 && !input1) || (!input0 && input1),
        }
    }
}
