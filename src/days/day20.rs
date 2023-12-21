use std::{fs, collections::{HashMap, BTreeMap}};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_20.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    Flipflop(bool),
    Conjunction,
    Broadcast,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pulse {
    High,
    Low,
}

struct Message<'a> {
    from: &'a str,
    to: &'a str,
    pulse: Pulse,
}

#[derive(Debug)]
struct Module<'a> {
    r#type: ModuleType,
    conn: Vec<&'a str>,
    memory: Option<HashMap<&'a str, Pulse>>,
}

fn parse_modules<'a>(input: &'a [String]) -> BTreeMap<&'a str, Module> {
    let mut modules = BTreeMap::<&str, Module>::new();
    let mut modules_conj = BTreeMap::<&str, Module>::new();

    input.iter()
        .for_each(|line| {
            let mut iter = line.split(" -> ");
            let part1 = iter.next().unwrap();
            let module_type = match part1.chars().nth(0).unwrap() {
                '%' => ModuleType::Flipflop(false),
                '&' => ModuleType::Conjunction,
                 _  => ModuleType::Broadcast,
            };
            let module_name = if module_type == ModuleType::Broadcast {part1} else {&part1[1..]};
            let connections: Vec<&str> = iter.next().unwrap().split(",").map(|s| s.trim()).collect();

            if module_type == ModuleType::Conjunction {
                modules_conj.insert(module_name, 
                               Module { r#type: module_type.clone(), 
                                   conn: connections, 
                                   memory: Some(HashMap::<&str, Pulse>::new()),
                               });
            } else {
                modules.insert(module_name, 
                               Module { r#type: module_type,
                                   conn: connections, 
                                   memory: None,
                               });
            }
        });

    //fill conjuction incomming connections
    modules_conj.iter_mut()
        .for_each(|(name, m)| {
            modules.iter()
                .filter(|(_, v)| v.conn.contains(name))
                .for_each(|(k, _)| {
                    if let Some(mem) = m.memory.as_mut() {
                        mem.insert(*k, Pulse::Low);
                    }
                });
        });

    modules.extend(modules_conj);
    modules
}


fn solve_part_1(input: Vec<String>) -> u32 {
    let mut modules = parse_modules(&input);
    let mut num_pulses_per_buttonpress: Vec<(u32, u32)> = vec![];
    let initial_state = get_state(&modules);
    
    for _ in 0..1000 {
        let mut count_high = 0;
        let mut count_low = 0;
        let mut pulse_queue: Vec<Message> = vec![Message{from: "button", to: "broadcaster", pulse: Pulse::Low}];

        while !pulse_queue.is_empty() {
            let message = pulse_queue.remove(0);
            match message.pulse {
                Pulse::Low => count_low += 1,
                Pulse::High => count_high += 1,
            }

            if let Some(receiver) = modules.get_mut(message.to) {
                match receiver.r#type {
                    ModuleType::Broadcast => {
                        receiver.conn.iter()
                            .for_each(|conn| {
                                pulse_queue.push(Message { from: message.to, to: conn, pulse: message.pulse.clone() } );
                            })
                    },
                    ModuleType::Flipflop(switched_on) => {
                        if message.pulse == Pulse::Low {
                            if switched_on {
                                receiver.conn.iter()
                                    .for_each(|conn| {
                                        pulse_queue.push(Message { from: message.to, to: conn, pulse: Pulse::Low} );
                                    })
                            } else {
                                receiver.conn.iter()
                                    .for_each(|conn| {
                                        pulse_queue.push(Message { from: message.to, to: conn, pulse: Pulse::High} );
                                    })
                            }
                            receiver.r#type = ModuleType::Flipflop(!switched_on);
                        }
                    },
                    ModuleType::Conjunction =>  {
                        receiver.memory.as_mut().unwrap().insert(message.from, message.pulse.clone());
                        let send_pulse = match receiver.memory.as_ref().unwrap().iter().all(|(_, v)| *v == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        receiver.conn.iter()
                            .for_each(|conn| {
                                pulse_queue.push(Message { from: message.to, to: conn, pulse: send_pulse.clone()} );
                            })
                    },
                };
            };
        }

        num_pulses_per_buttonpress.push((count_high, count_low));
    
        if get_state(&modules) == initial_state {
            break;
        }
    }

    let num_full_cycles: u32 = 1000 / num_pulses_per_buttonpress.len() as u32;
    let reminder = 1000 % num_pulses_per_buttonpress.len();

    let mut counts = num_pulses_per_buttonpress.iter()
            .fold((0, 0), |acc, (h, l)| {
                (acc.0 + h, acc.1 + l)
            });

    counts.0 *= num_full_cycles;
    counts.1 *= num_full_cycles;

    (0..reminder).for_each(|i| {
        counts.0 += num_pulses_per_buttonpress[i].0;
        counts.1 += num_pulses_per_buttonpress[i].1;
    });

    counts.0*counts.1
}

fn get_state(modules: &BTreeMap<&str, Module<'_>>) -> String {
    modules.iter()
        .filter(|(_, module)| match module.r#type { 
                ModuleType::Conjunction | ModuleType::Flipflop(_) => true,
                _ => false, 
        })
        .map(|(_, m)| {
            match m.r#type {
                ModuleType::Conjunction => {
                    if m.memory.as_ref().unwrap().iter().all(|(_, v)| *v == Pulse::Low) {
                        '0'
                    } else {
                        '1'
                    }
                },
                ModuleType::Flipflop(on) => {
                    match on {
                        true => '1',
                        false => '0',
                    }
                },
                _ => panic!("Should be filtered out!"),
            }

        })
        .collect::<String>()
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let mut modules = parse_modules(&input);
    let initial_state = get_state(&modules);
    
    let mut button_press_counter = 0; 
    loop {
        button_press_counter += 1;

        let mut num_low_rx_pulses = 0;
        let mut pulse_queue: Vec<Message> = vec![Message{from: "button", to: "broadcaster", pulse: Pulse::Low}];
        while !pulse_queue.is_empty() {
            let message = pulse_queue.remove(0);

            if message.to == "rx" && message.pulse == Pulse::Low {
                num_low_rx_pulses += 1;
            }

            if let Some(receiver) = modules.get_mut(message.to) {
                match receiver.r#type {
                    ModuleType::Broadcast => {
                        receiver.conn.iter()
                            .for_each(|conn| {
                                pulse_queue.push(Message { from: message.to, to: conn, pulse: message.pulse.clone() } );
                            })
                    },
                    ModuleType::Flipflop(switched_on) => {
                        if message.pulse == Pulse::Low {
                            if switched_on {
                                receiver.conn.iter()
                                    .for_each(|conn| {
                                        pulse_queue.push(Message { from: message.to, to: conn, pulse: Pulse::Low} );
                                    })
                            } else {
                                receiver.conn.iter()
                                    .for_each(|conn| {
                                        pulse_queue.push(Message { from: message.to, to: conn, pulse: Pulse::High} );
                                    })
                            }
                            receiver.r#type = ModuleType::Flipflop(!switched_on);
                        }
                    },
                    ModuleType::Conjunction =>  {
                        receiver.memory.as_mut().unwrap().insert(message.from, message.pulse.clone());
                        let send_pulse = match receiver.memory.as_ref().unwrap().iter().all(|(_, v)| *v == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        receiver.conn.iter()
                            .for_each(|conn| {
                                pulse_queue.push(Message { from: message.to, to: conn, pulse: send_pulse.clone()} );
                            })
                    },
                };
            };
        }
        
        if num_low_rx_pulses == 1 {
            break;
        }

        if get_state(&modules) == initial_state {
            break;
        }
    }

    button_press_counter
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 32000000);
    }

    #[test]
    fn simple_case_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 11687500);
    }
}
