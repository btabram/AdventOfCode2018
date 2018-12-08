use std::fs;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

type ErrorHolder = Box<std::error::Error>;
type Steps = Vec<Step>;
type Requirements = HashMap<Step, HashSet<char>>;
type PendingSteps<'s, 'r> = Vec<(&'s Step, &'r HashSet<char>)>;

#[derive(Debug, Clone, Copy)]
struct Step {
    id: char,
    started: bool,
    duration: i32,
}

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Step) -> bool {
        self.id == other.id
    }
}

impl Eq for Step {}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Step {
    fn new(id: char) -> Step {
        // We want a duration of 61 for A, 62 for B etc. and 'A' hass 65 value ASCII
        let duration = (id as i32) - 4; 
        Step { id, started: false, duration }
    }
}

fn parse_input() -> Result<(Steps, Requirements), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;
    let mut steps = vec![];
    let mut requirements = HashMap::new();
    
    for line in input.lines() {
        let requirement_id = line.chars().skip(5).next().expect("Failed to parse requirement_id");
        let step_id = line.chars().skip(36).next().expect("Failed to parse step_id");

        // Track the full list of unique steps
        steps.push(Step::new(requirement_id));
        steps.push(Step::new(step_id));

        // Insert requirements to the map as apprpriate
        requirements.entry(Step::new(step_id)).or_insert(HashSet::new()).insert(requirement_id);
    }
    
    // Remove duplicate steps
    steps.sort();
    steps.dedup();

    // Ensure steps without any requirements are in the map 
    steps.iter().for_each(|&s| { requirements.entry(s).or_insert(HashSet::new()); });

    println!("{:?}", steps);

    Ok((steps, requirements))
}

fn get_sorted_pending_valid_steps(requirements: &Requirements) -> PendingSteps {
    let mut steps: Vec<_> = requirements.iter().filter(|(s, r)| r.len() == 0 && !s.started).collect();
    steps.sort_by_key(|(&s, _)| s);
    steps
}

fn find_next_step(requirements: &Requirements) -> Step {
    // Choose the next valid step which is first alphabetically
    let doable_steps = get_sorted_pending_valid_steps(&requirements);
    let (&next_step, _) = doable_steps[0];
    next_step
}

fn do_step(step: &Step, requirements: &Requirements) -> Requirements {
    let mut new_requirements = requirements.clone();
    new_requirements.remove(step);
    new_requirements.values_mut().for_each(|r| { r.remove(&step.id); });
    new_requirements
}

fn start_work(available_workers: &mut usize, steps: &Steps, requirements: &Requirements) -> Steps {
    let pending_steps = get_sorted_pending_valid_steps(&requirements);
    if pending_steps.len() == 0 { std::process::exit(0); }
    let mut new_steps = steps.clone();
    for i in 0..std::cmp::min(pending_steps.len(), *available_workers) {
        let (ps, _) = pending_steps[i];
        new_steps.iter_mut().filter(|s| *s == ps).for_each(
            |s| if !s.started {
                s.started = true;
                *available_workers -= 1;
            });
    }
    new_steps
}

fn advance_time(steps: &Steps) -> Steps {
    let mut new_steps = steps.clone();
    new_steps.iter_mut().filter(|s| s.started).for_each(|s| s.duration -= 1);
    new_steps
}

fn main() -> Result<(), ErrorHolder> {
    let (steps, requirements) = parse_input()?;

    println!("\n#### Part 1 ####\n\n");
    part_1(&steps, &requirements);

    println!("\n#### Part 2 ####\n\n");
    part_2(&steps, &requirements);

    Ok(())
}

fn part_1(steps: &Steps, requirements: &Requirements) {
    println!("The initial requirements map is: {:?}\n", requirements);
    let mut steps_taken = vec![];
    let mut requirements_mut = requirements.clone();
    for _ in 0..steps.len() {
        let next_step = find_next_step(&requirements_mut);
        steps_taken.push(next_step);

        requirements_mut = do_step(&next_step, &requirements_mut);
        println!("The new requirements_mut map after step {} is: {:?}", next_step.id, requirements_mut);
    }

    let steps_in_order: String = steps_taken.iter().map(|p| p.id).collect();
    println!("\nThe steps taken, in order, were '{}'", steps_in_order);
}

fn part_2(steps: &Steps, requirements: &Requirements) {
    let mut requirements_mut = requirements.clone();
    let mut steps_mut = steps.clone();
    let mut available_workers = 5usize;

    let mut time = 0;
    loop {
        steps_mut = start_work(&mut available_workers, &steps_mut, &requirements_mut);
        steps_mut = advance_time(&steps_mut);

        for completed_step in steps_mut.iter().filter(|s| s.duration == 0) {
            requirements_mut = do_step(&completed_step, &requirements_mut);
            available_workers += 1;
        }

        time += 1;

        if steps_mut.iter().all(|s| s.duration <= 0) {
            break;
        }
    }

    println!("The steps took {}s in total with {} workers!", time, available_workers);
}

