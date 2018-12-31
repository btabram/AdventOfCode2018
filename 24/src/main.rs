use std::fs;
use std::collections::{HashMap, HashSet};

extern crate failure;
use failure::{Error, format_err};

extern crate regex;
use regex::{Regex, Match};

type Army = HashMap<i32, Group>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DamageType {
    Bludgeoning,
    Slashing,
    Fire,
    Cold,
    Radiation,
}
use self::DamageType::*;

impl std::str::FromStr for DamageType {
    type Err = Error;

    fn from_str(s: &str) -> Result<DamageType, Self::Err> {
        match s {
            "bludgeoning" => Ok(Bludgeoning),
            "slashing" => Ok(Slashing),
            "fire" => Ok(Fire),
            "cold" => Ok(Cold),
            "radiation" => Ok(Radiation),
            _ => Err(format_err!("Failed to parse {} as DamageType enum", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    units: i32,
    hp: i32,
    immune: HashSet<DamageType>,
    weak: HashSet<DamageType>,
    damage: i32,
    dt: DamageType,
    initiative: i32,
    id: i32,
    target: Option<Option<i32>>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.units * self.damage
    }

    fn calculate_damage(&self, other: &Group) -> i32 {
        if other.immune.contains(&self.dt) {
            0
        }
        else if other.weak.contains(&self.dt) {
            self.effective_power() * 2
        }
        else {
            self.effective_power()
        }
    }

    fn resolve_damage(&mut self, damage: i32) {
        let excess_damage = damage % self.hp;
        let actual_damage = damage - excess_damage;
        let losses = actual_damage / self.hp;
        self.units = std::cmp::max(0, self.units - losses);
    }

    fn reset_target(&mut self) {
        self.target = None;
    }

    fn find_target(&mut self, enemy_army: &Army, taken_targets: &HashSet<i32>) {
        let targets: Vec<_> = enemy_army
                                .values()
                                .filter(|g| !taken_targets.contains(&g.id))
                                .collect();

        let mut target = None;
        let mut max_damage = -1;
        for t in &targets {
            let potential_damage = self.calculate_damage(t);

            if potential_damage > max_damage {
                max_damage = potential_damage;
                target = Some(t);
            }
            else if potential_damage == max_damage {
                // Use effective power as a tie breaker
                let current_power = target.unwrap().effective_power();
                if t.effective_power() > current_power {
                    target = Some(t);
                }
                else if t.effective_power() == current_power {
                    // Use initiative as a further tie breaker
                    if t.initiative > target.unwrap().initiative {
                        target = Some(t);
                    }
                }
            }
        }

        if max_damage > 0 {
            self.target = Some(Some(target.unwrap().id));
        }
        else {
            // Outer option shows if we've chosen a target, however the target
            // can be None hence the inner option.
            self.target = Some(None);
        }
    }
}

fn select_all_targets(army: &mut Army, enemy_army: &Army) {
    for g in army.values_mut() {
        g.reset_target();
    }

    let mut target_taken = HashSet::new();

    loop {
        let mut next_group_to_choose = None;
        let mut max_power = 0;
        let mut max_power_initiative = 0;

        for (id, g) in army.clone() {
            // Already chosen a target
            if g.target != None {
                continue;
            }

            let power = g.effective_power();
            if power > max_power {
                max_power = power;
                max_power_initiative = g.initiative;
                next_group_to_choose = Some(id);
            }
            else if power == max_power {
                // Use initiative to break ties
                if g.initiative > max_power_initiative {
                    max_power_initiative = g.initiative;
                    next_group_to_choose = Some(id);
                }
            }
        }

        // All groups have chosen a target
        if next_group_to_choose == None {
            break;
        }

        let g = army.get_mut(&next_group_to_choose.unwrap()).unwrap();
        assert!(g.target == None);
        g.find_target(enemy_army, &target_taken);
        assert!(g.target != None);

        match g.target.unwrap() {
            None => {},
            Some(target) => {
                target_taken.insert(target);
            },
        }
    }
}

fn do_turn(army1: &mut Army, army2: &mut Army) {
    select_all_targets(army1, army2);
    select_all_targets(army2, army1);

    let mut all_groups = vec![];
    army1.values().for_each(|g| all_groups.push(g.clone()));
    army2.values().for_each(|g| all_groups.push(g.clone()));

    // Order by decreasing initiative
    all_groups.sort_unstable_by_key(|g| 999999 - g.initiative);

    // Resolve attacks in order of decreasing initiative
    for ref id in all_groups.iter().map(|g| g.id) {
        // Get a ref to the attacking group
        let group = if army1.contains_key(id) {
            army1.get(id).expect("Unknown group id").clone()
        }
        else {
            army2.get(id).expect("Unknown group id").clone()
        };

        // Skip this group it's got no target or no units
        let target_id_option = group.target.unwrap();
        if target_id_option == None || group.units == 0 {
            continue;
        }

        // Get a mut ref to the attacking group
        let ref target_id = target_id_option.unwrap();
        let ref mut target = if army1.contains_key(target_id) {
            army1.get_mut(target_id).expect("Unknown group id")
        }
        else {
            army2.get_mut(target_id).expect("Unknown group id")
        };

        // Resolve the attck itself
        let damage = group.calculate_damage(target);
        target.resolve_damage(damage);
    }

    // Remove any defeated groups
    army1.retain(|_, g| g.units > 0);
    army2.retain(|_, g| g.units > 0);
}

fn parse_damange_types(m: Option<Match>) -> Result<HashSet<DamageType>, Error> {
    match m {
        None => Ok(HashSet::new()),
        Some(m) => {
            let s = m.as_str();
            let mut set = HashSet::new();
            for dt in s.split(", ") {
                set.insert(dt.parse()?);
            }
            Ok(set)
        },
    }
}

fn parse_army(re: &Regex, s: &str, id: i32) -> Result<Group, Error> {
    let c =re.captures(s).expect("Regex failed to match input");

    let units: i32 = c.name("units").unwrap().as_str().parse()?;
    let hp: i32 = c.name("hp").unwrap().as_str().parse()?;
    // There are two different possible immune captues in the regex since
    // immunities and weakness can be specified in any order
    let mut immune = parse_damange_types(c.name("immune1"))?;
    if immune.is_empty() {
        immune = parse_damange_types(c.name("immune2"))?;
    }
    let weak = parse_damange_types(c.name("weak"))?;
    let damage: i32 = c.name("damage").unwrap().as_str().parse()?;
    let dt: DamageType = c.name("damage_type").unwrap().as_str().parse()?;
    let initiative: i32 = c.name("initiative").unwrap().as_str().parse()?;

    Ok(Group {
        units, hp, immune, weak, damage, dt, initiative, id, target: None
    })
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;

    let input_regex = Regex::new(r"^(?P<units>\d+) units each with (?P<hp>\d+) hit points (?:\((?:immune to (?P<immune1>[^;]+))?(?:; )?(?:weak to (?P<weak>[^;]+))?(?:; )?(?:immune to (?P<immune2>[^;]+))?\) )?with an attack that does (?P<damage>\d+) (?P<damage_type>\w+) damage at initiative (?P<initiative>\d+)$")?;

    let mut immune_army = HashMap::new();
    let mut infection_army = HashMap::new();

    let mut id = 0;
    // Assume the immune system armies are listed first
    let mut infection = false;
    for line in input.lines() {
        match line {
            "Immune System:" | "" => {},
            "Infection:" => infection = true,
            _ => {
                let a = parse_army(&input_regex, line, id)?;
                if infection {
                    infection_army.insert(id, a);
                }
                else {
                    immune_army.insert(id, a);
                }
                id += 1;
            },
        }
    }

    part1(&immune_army, &infection_army);
    part2(&immune_army, &infection_army);

    Ok(())
}

fn part1(initial_immune_army: &Army, initial_infection_army: &Army) {
    let mut immune_army = initial_immune_army.clone();
    let mut infection_army = initial_infection_army.clone();

    while immune_army.len() > 0 && infection_army.len() > 0 {
        do_turn(&mut immune_army, &mut infection_army);
    }

    if infection_army.len() > 0 {
        let sum: i32 = infection_army.values().map(|g| g.units).sum();
        println!("The infection wins with {} units remaining in part 1.",
                    sum);
    }
    else {
        let sum: i32 = immune_army.values().map(|g| g.units).sum();
        println!("The immune system wins with {} units remaining in part 1.",
                    sum);
    }
}

fn part2(initial_immune_army: &Army, initial_infection_army: &Army) {
    let mut boost = 1;
    loop {
        let mut immune_army = initial_immune_army.clone();
        let mut infection_army = initial_infection_army.clone();

        // Apply the boost
        immune_army.values_mut().for_each(|g| g.damage += boost);

        // Resolve the fight between the armies. In some cases the fight will
        // go on forever because there will be two goups left who cannot deal
        // damage to one another because of immunities. After 10000 iterations
        // assume that that's happened
        let mut j = 0;
        while immune_army.len() > 0 && infection_army.len() > 0 && j < 10000 {
            do_turn(&mut immune_army, &mut infection_army);
            j += 1;
        }

        if infection_army.len() > 0 && immune_army.len() > 0 {
            // println!("A boost of {} results in ENDLESS WAR!", boost);
        }
        else if infection_army.len() > 0 {
            /*
            let sum: i32 = infection_army.values().map(|g| g.units).sum();
            println!("With boost {} the infection wins with {} units.",
                     boost, sum);
            */
        }
        else {
            let sum: i32 = immune_army.values().map(|g| g.units).sum();
            println!("With boost {} the immune system wins with {} units.",
                     boost, sum);
            break;
        }

        boost += 1;
    }
}

