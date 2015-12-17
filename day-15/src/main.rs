use std::cmp::max;

struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

const SPRINKLES: Ingredient = Ingredient {
    capacity: 5,
    durability: -1,
    flavor: 0,
    texture: 0,
    calories: 5,
};

const PEANUT_BUTTER: Ingredient = Ingredient {
    capacity: -1,
    durability: 3,
    flavor: 0,
    texture: 0,
    calories: 1,
};

const FROSTING: Ingredient = Ingredient {
    capacity: 0,
    durability: -1,
    flavor: 4,
    texture: 0,
    calories: 6,
};

const SUGAR: Ingredient = Ingredient {
    capacity: -1,
    durability: 0,
    flavor: 0,
    texture: 2,
    calories: 8,
};

const BUTTERSCOTCH: Ingredient = Ingredient {
    capacity: -1,
    durability: -2,
    flavor: 6,
    texture: 3,
    calories: 8,
};

const CINNAMON: Ingredient = Ingredient {
    capacity: 2,
    durability: 3,
    flavor: -2,
    texture: -1,
    calories: 3,
};

fn get_sum<T>(ingredients: &[(isize, &Ingredient)], f: T) -> usize
    where T: Fn(&Ingredient) -> isize
{
    max(0,
        ingredients.iter()
                   .map(|&(factor, ingredient)| factor * f(ingredient))
                   .fold(0, |a, i| a + i)) as usize
}

fn get_score(ingredients: &[(isize, &Ingredient)]) -> usize {
    let capacity = get_sum(ingredients, |ingredient| ingredient.capacity);
    let durability = get_sum(ingredients, |ingredient| ingredient.durability);
    let flavor = get_sum(ingredients, |ingredient| ingredient.flavor);
    let texture = get_sum(ingredients, |ingredient| ingredient.texture);
    capacity * durability * flavor * texture
}

fn get_calories(ingredients: &[(isize, &Ingredient)]) -> usize {
    get_sum(ingredients, |ingredient| ingredient.calories)
}

fn get_optimal_score() -> usize {
    let mut max_score = 0;
    for sprinkles in 0..101 {
        let remaining = 100 - sprinkles;
        for peanut_butter in 0..remaining + 1 {
            let remaining = remaining - peanut_butter;
            for frosting in 0..remaining + 1 {
                let sugar = remaining - frosting;

                let score = get_score(&[(sprinkles, &SPRINKLES),
                                        (peanut_butter, &PEANUT_BUTTER),
                                        (frosting, &FROSTING),
                                        (sugar, &SUGAR)]);
                
                max_score = max(max_score, score);
            }
        }
    }
    max_score
}

fn get_limited_score() -> usize {
    let mut max_score = 0;
    for sprinkles in 0..101 {
        let remaining = 100 - sprinkles;
        for peanut_butter in 0..remaining + 1 {
            let remaining = remaining - peanut_butter;
            for frosting in 0..remaining + 1 {
                let sugar = remaining - frosting;

                let ingredients = [(sprinkles, &SPRINKLES),
                                   (peanut_butter, &PEANUT_BUTTER),
                                   (frosting, &FROSTING),
                                   (sugar, &SUGAR)];

                let score = get_score(&ingredients);
                let calories = get_calories(&ingredients);

                if calories == 500 {
                    max_score = max(max_score, score);
                }
            }
        }
    }
    max_score
}

fn main() {
    let optimal_score = get_optimal_score();
    println!("Optimal Score: {}", optimal_score);

    let limited_score = get_limited_score();
    println!("Limited Score: {}", limited_score);
}

#[test]
fn test_score() {
    let score = get_score(&[(44, &BUTTERSCOTCH), (56, &CINNAMON)]);
    assert_eq!(score, 62842880);
}

#[test]
fn test_calories() {
    let ingredients = [(40, &BUTTERSCOTCH), (60, &CINNAMON)];
    assert_eq!(get_calories(&ingredients), 500);
    assert_eq!(get_score(&ingredients), 57600000);
}
