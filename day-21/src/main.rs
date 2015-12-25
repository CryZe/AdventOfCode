pub mod game;

use std::cmp::{min, max};
use game::prelude::*;
use game::{Boss, Equipment, FightResult, Player, Shop};

fn get_all_equipments(shop: &Shop) -> Vec<Equipment> {
    let mut equipments = Vec::new();
    for weapon in shop.weapons
                      .into_iter()
                      .cloned() {
        for armor in shop.armors
                         .into_iter()
                         .cloned()
                         .map(|i| Some(i))
                         .chain([None].iter().cloned()) {
            for left_ring in shop.rings
                                 .into_iter()
                                 .cloned()
                                 .map(|i| Some(i))
                                 .chain([None].iter().cloned()) {
                for right_ring in shop.rings
                                      .into_iter()
                                      .cloned()
                                      .map(|i| Some(i))
                                      .chain([None].iter().cloned()) {
                    if left_ring != right_ring {
                        let equipment = Equipment::new(weapon, armor, left_ring, right_ring);
                        equipments.push(equipment);
                    }
                }
            }
        }
    }
    equipments
}

fn find_min_and_win<F: Fighter>(player: &mut Player, enemy: &mut F, shop: &Shop) -> Option<usize> {
    let mut min_cost = None;

    for equipment in get_all_equipments(shop) {
        let cost = equipment.get_cost();

        player.set_equipment(equipment);

        let result = player.fight(enemy);

        if result == FightResult::Win {
            min_cost = Some(min_cost.map_or(cost, |c| min(c, cost)));
        }
    }

    min_cost
}

fn find_max_and_lose<F: Fighter>(player: &mut Player, enemy: &mut F, shop: &Shop) -> Option<usize> {
    let mut min_cost = None;

    for equipment in get_all_equipments(shop) {
        let cost = equipment.get_cost();

        player.set_equipment(equipment);

        let result = player.fight(enemy);

        if result == FightResult::Loss {
            min_cost = Some(min_cost.map_or(cost, |c| max(c, cost)));
        }
    }

    min_cost
}

fn main() {
    let shop = Shop::default();
    let mut player = Player::default();
    let mut enemy = Boss::default();

    let min_and_win = find_min_and_win(&mut player, &mut enemy, &shop);

    match min_and_win {
        Some(cost) => println!("The lowest amount of Gold needed to win is {}.", cost),
        None => println!("No equipment could defeat the enemy."),
    }

    let max_and_lose = find_max_and_lose(&mut player, &mut enemy, &shop);

    match max_and_lose {
        Some(cost) => {
            println!("The highest amount of Gold needed to still lose is {}.",
                     cost)
        }
        None => println!("No equipment could lose to the enemy."),
    }
}
