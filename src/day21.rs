use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
struct Fighter {
    hp: usize,
    damages: usize,
    armor: usize,
}

#[derive(Debug)]
struct ArmoryItem {
    cost: usize,
    damage: usize,
    armor: usize,
}

static WEAPONS_LIST: &str = r#"Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0"#;

static ARMORS_LIST: &str = r#"Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5"#;

static RINGS_LIST: &str = r#"Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3"#;

impl FromStr for ArmoryItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let cost = parts[1].trim().parse().unwrap();
        let damage = parts[2].trim().parse().unwrap();
        let armor = parts[3].trim().parse().unwrap();
        Ok(Self {
            cost,
            damage,
            armor,
        })
    }
}

fn str_to_vec_armory_item(s: &str) -> Vec<ArmoryItem> {
    s.lines()
        .map(str::parse::<ArmoryItem>)
        .map(Result::unwrap)
        .collect()
}

impl Fighter {
    fn new(hp: usize, damages: usize, armor: usize) -> Self {
        Self { hp, damages, armor }
    }

    fn win_against(&self, boss: &Fighter) -> bool {
        let damages_by_turn = self.damages.saturating_sub(boss.armor).max(1);
        let boss_damages_by_turn = boss.damages.saturating_sub(self.armor).max(1);

        let turns_to_kill = boss.hp / damages_by_turn + usize::from(boss.hp % damages_by_turn != 0);
        let turns_to_be_killed =
            self.hp / boss_damages_by_turn + usize::from(self.hp % boss_damages_by_turn != 0);

        turns_to_kill <= turns_to_be_killed
    }
}

fn main() {
    let boss = Fighter::new(104, 8, 1);
    // Let's see winnign combination!

    let weapons: Vec<ArmoryItem> = str_to_vec_armory_item(WEAPONS_LIST);
    let armors = {
        let mut armors = str_to_vec_armory_item(ARMORS_LIST);
        // As we may take no armour, adding a no cost, no stats armour
        armors.push(ArmoryItem {
            damage: 0,
            cost: 0,
            armor: 0,
        });
        armors
    };
    let rings = str_to_vec_armory_item(RINGS_LIST);

    let mut ring_sets = vec![Vec::new()];
    for idx in 0..=2 {
        ring_sets.extend(rings.iter().permutations(idx));
    }

    let mut min_winning_cost = usize::MAX;
    let mut max_winning_cost = 0_usize;
    for weapon in &weapons {
        for armor in &armors {
            for set in &ring_sets {
                let cost: usize =
                    weapon.cost + armor.cost + set.iter().map(|item| item.cost).sum::<usize>();

                let damages = weapon.damage
                    + armor.damage
                    + set.iter().map(|item| item.damage).sum::<usize>();
                let a =
                    weapon.armor + armor.armor + set.iter().map(|item| item.armor).sum::<usize>();

                if Fighter::new(100, damages, a).win_against(&boss) {
                    min_winning_cost = cost.min(min_winning_cost);
                } else {
                    max_winning_cost = cost.max(max_winning_cost);
                }
            }
        }
    }

    println!("Part 1: {}", min_winning_cost);
    println!("Part 2: {}", max_winning_cost);
}
