use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Fighter {
    hp: usize,
    damages: usize,
    armor: usize,
}

impl Fighter {
    fn new(hp: usize, damages: usize, armor: usize) -> Self {
        Self { hp, damages, armor }
    }

    fn win_against(&self, boss: &Fighter) -> bool {
        let damages_by_turn = self.damages.saturating_sub(boss.armor).max(1);
        let boss_damages_by_turn = boss.damages.saturating_sub(self.armor).max(1);

        let turns_to_kill =
            boss.hp / damages_by_turn + if boss.hp % damages_by_turn != 0 { 1 } else { 0 };
        let turns_to_be_killed = self.hp / boss_damages_by_turn
            + if self.hp % boss_damages_by_turn != 0 {
                1
            } else {
                0
            };

        // println!("{:?} => {} {}", self, turns_to_kill, turns_to_be_killed);
        turns_to_kill <= turns_to_be_killed
    }
}

fn main() {
    // Looking to the shop item,
    // Damages are between 4 to 14
    // Armor is between 1 to 11

    let boss = Fighter::new(104, 8, 1);
    // Let's see winnign combination!

    for dmg in 4..=14 {
        for armor in 1..=11 {
            let player = Fighter::new(100, dmg, armor);
            if player.win_against(&boss) {
                println!("Damages: {}, Armor: {} is ok", dmg, armor);
            }
        }
    }

    // println!("{}", player.win_against(&boss));
}
