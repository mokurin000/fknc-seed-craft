use std::collections::HashMap;

use crate::model::{Seed, Solution};

fn reward_level(total_price: i64) -> (i32, i32) {
    if total_price > 1_200_000 {
        (25, 4)
    } else if total_price > 500_000 {
        (5, 3)
    } else if total_price > 25_000 {
        (1, 2)
    } else if total_price > 0 {
        (0, 1)
    } else {
        (0, 0)
    }
}

fn expand_inventory(seeds: &[Seed]) -> Vec<Seed> {
    let mut expanded = Vec::new();
    for seed in seeds {
        for _ in 0..seed.count {
            expanded.push(seed.clone());
        }
    }
    expanded
}

pub fn solve(seeds: &[Seed]) -> Option<Solution> {
    const SCALE: i64 = 100;

    let items = expand_inventory(seeds);
    let prices: Vec<i64> = items.iter().map(|s| s.price / SCALE).collect();
    let max_sum: usize = prices.iter().map(|&p| p as usize).sum();

    let mut dp = vec![vec![false; max_sum + 1]; 6];
    let mut parent = vec![vec![None; max_sum + 1]; 6];

    dp[0][0] = true;

    for (idx, &price) in prices.iter().enumerate() {
        let price_usize = price as usize;
        for k in (0..5).rev() {
            for s in (0..=max_sum - price_usize).rev() {
                if !dp[k][s] {
                    continue;
                }

                let ns = s + price_usize;

                if !dp[k + 1][ns] {
                    dp[k + 1][ns] = true;
                    parent[k + 1][ns] = Some((k, s, idx));
                }
            }
        }
    }

    let mut best: Option<(i32, i32, i64, usize, usize)> = None; // (prob, energy, -total, k, s)

    for (s, valid) in dp[5].iter().enumerate() {
        if !valid {
            continue;
        }

        let total_price = (s as i64) * SCALE;
        let (probability, energy) = reward_level(total_price);

        let score = (probability, energy, -total_price);

        match best {
            None => {
                best = Some((probability, energy, -total_price, 5, s));
            }
            Some((p, e, neg_t, _, _)) => {
                if score > (p, e, neg_t) {
                    best = Some((probability, energy, -total_price, 5, s));
                }
            }
        }
    }

    let (_, _, _, k, s) = best?;

    let mut chosen_items = Vec::new();
    let mut ck = k;
    let mut cs = s;

    while ck > 0 {
        if let Some((pk, ps, idx)) = parent[ck][cs] {
            chosen_items.push(items[idx].clone());
            ck = pk;
            cs = ps;
        } else {
            break;
        }
    }

    chosen_items.reverse();

    let total_price = chosen_items.iter().map(|s| s.price).sum();
    let (probability, energy) = reward_level(total_price);

    let mut counts: HashMap<String, usize> = HashMap::new();
    for seed in &chosen_items {
        *counts.entry(seed.name.clone()).or_insert(0) += 1;
    }

    Some(Solution {
        price: total_price,
        probability,
        energy,
        counts,
        chosen: chosen_items,
    })
}
