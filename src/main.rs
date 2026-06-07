use std::cmp::Reverse;

use crate::algo::solve;
use crate::model::Seed;

mod algo;
mod model;

fn format_money(x: i64) -> String {
    let mut s = x.to_string();
    let mut i = s.len();
    while i > 3 {
        i -= 3;
        s.insert(i, ',');
    }
    s
}

fn main() {
    let mut seeds = vec![
        // 普通种子
        Seed::new("草莓", 1_250_000, 5),
        Seed::new("向日葵", 3_000_000, 5),
        Seed::new("猕猴桃", 600_000, 5),
        Seed::new("香蕉", 300_000, 5),
        Seed::new("苹果", 150_000, 5),
        Seed::new("玉米", 90_000, 5),
        Seed::new("西瓜", 60_000, 5),
        Seed::new("竹子", 30_000, 5),
        Seed::new("黄瓜", 9_000, 5),
        Seed::new("波斯菊", 2_000, 5),
        Seed::new("番茄", 100, 5),
        Seed::new("土豆", 0, 5),
        // 月球种子
        Seed::new("液光藤", 3_100_000, 5),
        Seed::new("月核树", 2_000_000, 5),
        Seed::new("星叶菜", 500_000, 5),
        Seed::new("月莓", 300_000, 5),
        Seed::new("银灰苔", 120_000, 5),
        Seed::new("月环树", 60_000, 5),
        Seed::new("月番茄", 16_000, 5),
        Seed::new("月灯草", 8_000, 5),
        Seed::new("灰壤豆", 1_000, 5),
        Seed::new("月光草", 0, 5),
    ];

    // 按价格降序排序
    seeds.sort_by_key(|s| Reverse(s.price));

    if let Some(result) = solve(&seeds) {
        println!("=== 最优方案 ===");
        println!();
        println!("总价: {}", format_money(result.price));
        println!("出金概率: +{}%", result.probability);
        println!("能量增长: +{}", result.energy);
        println!();
        println!("选择种子:");

        let mut count_vec: Vec<(&String, &usize)> = result.counts.iter().collect();
        count_vec.sort_by_key(|(name, _)| name.as_str());

        for (name, count) in count_vec {
            println!("  {:<8} x{}", name, count);
        }

        println!();
        println!("详细列表:");

        for seed in &result.chosen {
            println!("  {:<8}{}", seed.name, format_money(seed.price));
        }
    } else {
        println!("无可行方案");
    }
}
