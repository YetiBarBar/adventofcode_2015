/****

boss = { hp: 71, damage: 10 }
player = { hp: 50, mana: 500 }

hard: 1937
* Shield -> Recharge -> Poison -> Shield -> Recharge -> Poison -> Shield -> Recharge -> Poison -> Shield -> Magic Missile -> Poison -> Magic Missile

easy: 1824
* Poison -> Recharge -> Shield -> Poison -> Recharge -> Shield -> Poison -> Recharge -> Shield -> Magic Missile -> Poison -> Magic Missile
* Poison -> Recharge -> Shield -> Poison -> Recharge -> Shield -> Poison -> Recharge -> Shield -> Poison -> Magic Missile -> Magic Missile
* Recharge -> Poison -> Shield -> Recharge -> Poison -> Shield -> Recharge -> Poison -> Shield -> Magic Missile -> Poison -> Magic Missile

***/
static CASTS: [Cast; 5] = [
    Cast::Drain,
    Cast::MagicMissile,
    Cast::Poison,
    Cast::Recharge,
    Cast::Shield,
];
#[derive(Debug, Clone)]
struct Player {
    hp: usize,
    mana: usize,
    applied_effects: Vec<Effect>,
}

impl Player {
    fn callable_cast(&self, boss: &Boss) -> Vec<Cast> {
        let by_mana_limit: Vec<Cast> = CASTS
            .iter()
            .filter(|cast| cast.cost() > self.mana)
            .cloned()
            .collect();
        todo!()
    }

    fn apply_effect(&mut self) {
        for effect in self.applied_effects.iter_mut() {
            effect.remaining_turns = effect.remaining_turns.saturating_sub(1);
            self.mana += effect.mana_recharge;
            self.hp = self.hp.saturating_sub(effect.damages);
        }

        self.applied_effects
            .retain(|item| item.remaining_turns != 0);
    }
}

#[derive(Debug, Clone)]
struct Effect {
    remaining_turns: usize,
    armor_bonus: usize,
    damages: usize,
    mana_recharge: usize,
    effect_type: EffectType,
}

#[derive(Clone)]
struct Boss {
    hp: usize,
    damage_by_turn: usize,
    applied_effects: Vec<Effect>,
}

impl Boss {
    fn attack(&self, player: &Player) -> usize {
        self.damage_by_turn
            .saturating_sub(
                player
                    .applied_effects
                    .iter()
                    .map(|eff| eff.armor_bonus)
                    .sum::<usize>(),
            )
            .min(1_usize)
    }
}

#[derive(Debug, Clone)]
enum EffectType {
    Shielded,
    Poisoned,
    Recharging,
}
#[derive(Debug, Clone)]
enum Cast {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Cast {
    fn cost(&self) -> usize {
        match self {
            Cast::MagicMissile => 53,
            Cast::Drain => 73,
            Cast::Shield => 113,
            Cast::Poison => 173,
            Cast::Recharge => 229,
        }
    }

    fn damages(&self) -> usize {
        match self {
            Cast::MagicMissile => 4,
            Cast::Drain => 2,
            _ => 0,
        }
    }

    fn effects(&self) -> Option<Effect> {
        match self {
            Cast::Shield => Some(Effect {
                remaining_turns: 6,
                armor_bonus: 7,
                damages: 0,
                mana_recharge: 0,
                effect_type: EffectType::Shielded,
            }),
            Cast::Poison => Some(Effect {
                remaining_turns: 6,
                armor_bonus: 0,
                damages: 3,
                mana_recharge: 0,
                effect_type: EffectType::Poisoned,
            }),
            Cast::Recharge => Some(Effect {
                remaining_turns: 5,
                armor_bonus: 0,
                damages: 0,
                mana_recharge: 103,
                effect_type: EffectType::Recharging,
            }),
            _ => None,
        }
    }

    fn heal(&self) -> usize {
        if matches!(self, Cast::Drain) {
            2
        } else {
            0
        }
    }
}

fn play_player_turn(player: &mut Player) {}

fn main() {
    let boss = Boss {
        hp: 71,
        damage_by_turn: 10,
        applied_effects: vec![],
    };

    todo!()
}
