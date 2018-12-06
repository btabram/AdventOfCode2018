use std::fs;
use std::collections::BTreeMap;

// Convert a &str to a i32 whilst removing any unwanted characters
fn get_i32(s: &str) -> i32 {
    let mut local_s = s.to_string();
    local_s.retain(|c| c != '#');
    return local_s.parse().expect("Failed to parse string as i32");
}

fn get_time(s: &str) -> i32 {
    let hour_string: String = s.chars().take(2).collect();
    let hour: i32 = hour_string.parse().expect("Failed to parse hours");

    let min_string: String = s.chars().skip(3).take(2).collect();
    let min: i32 = min_string.parse().expect("Failed to parse minutes");

    // Times are either 00:XX or 23:XX. We only care about their ordering
    if hour == 0 { min } else { min - 60 }
}

#[derive(Debug)]
enum Observation {
    FallAsleep,
    WakeUp,
    GuardId(i32),
}

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    // Use BTreeMap so that the keys are ordered
    let mut days: BTreeMap<String, Vec<(i32, Observation)>> = BTreeMap::new();
    let mut guards_times = BTreeMap::new();

    // Parse all the observations
    for line in input.lines().map(|l| l.to_string()) {
        let split_line: Vec<&str> = line.split(" ").collect();

        let date: String = split_line[0].chars().skip(1).collect();
        let time = get_time(split_line[1]);
        let observation = match split_line[2] {
            "falls" => Observation::FallAsleep,
            "wakes" => Observation::WakeUp,
            "Guard" => Observation::GuardId(get_i32(split_line[3])),
            _ => { println!("Unexpected observation!"); std::process::exit(1); },
        };

        let day = days.entry(date).or_insert(vec![]);
        day.push((time, observation));
    }

    // Iterate through the dates so we can move events before midnight to the correct 'day'
    // (putting them the day after they really are with a negative time to make things simpler)
    let mut previously_moved_events: Vec<(i32, Observation)> = vec![];
    for (_, value) in &mut days {

        // Find out which events should move
        let mut events_to_move_indexes = vec![];
        for (i, v) in value.iter().enumerate() {
            let (time, _) = *v;
            // We should move this event to the day after
            if time < 0 {
                events_to_move_indexes.push(i);
            }
        }

        // Remove these events and put them in a temporary vector
        let mut events_to_move = vec![];
        for i in events_to_move_indexes {
            events_to_move.push(value.remove(i));
        }

        // Now put any events which we've moved from the previous day into this day
        for event in previously_moved_events.drain(..) {
            value.push(event);
        }

        // Put the events which are being moved into the persistent vector
        previously_moved_events = events_to_move;
    }

    // Order the events by time
    for mut timed_observations in days.values_mut() {
        timed_observations.sort_unstable_by(|(t1, _), (t2, _)| t1.cmp(t2));  

        // Sanity check that there's no duplicate times
        let mut last_t = None;
        for (t, _) in timed_observations {
            let this_t = Some(t);
            if this_t == last_t {
                println!("Duplicate time detected!");
                std::process::exit(1);
            }
            last_t = this_t;
        }
    }

    let mut current_guard = None;
    let mut fell_asleep_time: Option<i32> = None;
    for (day, value) in &days {
        println!("{}: {:?}", day, value);
        for v in value {
            let (time, ref observation) = *v;
            match observation {
                Observation::GuardId(id) => current_guard = Some(id),
                Observation::FallAsleep => fell_asleep_time = Some(time),
                Observation::WakeUp => {
                    let id = current_guard.expect("Didn't find guard on duty!");
                    let fell_asleep = fell_asleep_time.expect("Didn't find when a guard fell asleep!");

                    let times = guards_times.entry(id).or_insert(BTreeMap::new());
                    for i in fell_asleep..time {
                        let t = times.entry(i).or_insert(0);
                        *t += 1;
                    }

                    fell_asleep_time = None;
                },
            }
        }

        // Check we've woken up after every sleep for a given day
        assert!(fell_asleep_time == None);
    }

    // Part 1
    {
        // Find the guard who was asleep most in total
        let (&id, _) = guards_times.iter().max_by(
                        |(_, times1), (_, times2)|
                            times1.values().sum::<u32>().cmp(
                                &times2.values().sum()
                            )
                        ).expect("Failed to find max...");
        println!("\nGuard {} sleeps the most in total!", id);
        let (min, sleep) = guards_times.get(&id).unwrap().iter().max_by(|(_, m1), (_, m2)| m1.cmp(m2)).unwrap();
        println!("Guard {} was asleep most during minute {}. They were asleep {} times!", id, min, sleep);
        println!("The answer to part 1 is {}", id * min);
    }

    // Part 2
    {
        // Find the guard who was asleep most frequently at any one minute
        let (&id, _) = guards_times.iter().max_by(
                        |(_, times1), (_, times2)|
                            times1.values().max().cmp(
                                &times2.values().max()
                            )
                        ).expect("Failed to find max...");
        println!("\nGuard {} sleeps the most frequently at any one minte!", id);
        let (min, sleep) = guards_times.get(&id).unwrap().iter().max_by(|(_, m1), (_, m2)| m1.cmp(m2)).unwrap();
        println!("Guard {} was asleep most during minute {}. They were asleep {} times!", id, min, sleep);
        println!("The answer to part 2 is {}", id * min);
    }

    Ok(())
}
