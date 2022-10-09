/*
Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9
Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1
Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8
*/

#[derive(Debug)]
pub struct Ingredients {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

macro_rules! ingredients {
    ($(capacity $capacity:literal, durability $durability:literal, flavor $flavor:literal, texture $texture:literal, calories $calories:literal),+ $(,)?) => {
        [$(

            Ingredients {
                capacity: $capacity,
                durability: $durability,
                flavor: $flavor,
                texture: $texture,
                calories: $calories,
            },
        )+]
    };
}

fn repartitions(max: isize, amount: isize) -> Vec<Vec<isize>> {
    if amount == 1 {
        return vec![vec![max]];
    }

    (0..max + 1)
        .flat_map(|x| {
            let mut retval = repartitions(max - x, amount - 1);

            for combination in &mut retval {
                combination.push(x);
            }
            retval
        })
        .collect::<Vec<_>>()
}

fn evaluate(repartition: &[isize], ingredients: &[Ingredients]) -> isize {
    assert!(repartition.len() == ingredients.len());
    let formula = repartition
        .iter()
        .zip(ingredients.iter())
        .map(|(quantity, item)| {
            (
                quantity * item.capacity,
                quantity * item.durability,
                quantity * item.flavor,
                quantity * item.texture,
            )
        })
        .fold((0, 0, 0, 0), |(a, b, c, d), (e, f, g, h)| {
            (a + e, b + f, c + g, d + h)
        });
    0.max(formula.0) * 0.max(formula.1) * 0.max(formula.2) * 0.max(formula.3)
}

fn calories(repartition: &[isize], ingredients: &[Ingredients]) -> isize {
    repartition
        .iter()
        .zip(ingredients.iter())
        .map(|(quantity, item)| quantity * item.calories)
        .sum()
}

pub fn main() {
    let ingredients = ingredients!(
        capacity 3, durability 0, flavor 0, texture -3, calories 2,
        capacity -3, durability 3, flavor 0, texture 0, calories 9,
        capacity -1, durability 0, flavor 4, texture 0, calories 1,
        capacity 0, durability 0, flavor -2, texture 2, calories 8,
    );

    /*     let ind = ingredients!(
        capacity -1, durability -2, flavor 6, texture 3, calories 8,
        capacity 2, durability 3, flavor -2, texture -1, calories 3,
    ); */

    println!(
        "Part 1: {:?}",
        repartitions(100, 4)
            .iter()
            .map(|v| evaluate(v, &ingredients))
            .max()
    );
    println!(
        "Part 2: {:?}",
        repartitions(100, 4)
            .iter()
            .filter(|v| calories(v, &ingredients) == 500)
            .map(|v| evaluate(v, &ingredients))
            .max()
    );
}
