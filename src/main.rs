use inquire::Text;
use std::cmp::Reverse;

use crate::algo::solve;
use crate::data::{FREE_SEEDS, SEEDS};
use crate::model::Seed;

mod algo;
mod data;
mod model;

fn format_money(x: u64) -> String {
    let mut s = x.to_string();
    let mut i = s.len();
    while i > 3 {
        i -= 3;
        s.insert(i, ',');
    }
    s
}

fn main() {
    println!("=== 设置每种种子的最大使用数量 ===");
    println!("（直接回车 = 使用默认值 5，数字大于5无意义）\n");

    let mut seed_objects = Vec::new();

    for (name, price) in SEEDS {
        loop {
            let prompt = format!("{} 的最大使用数量", name);

            let Ok(input) = Text::new(&prompt)
                .with_default("5")
                .with_help_message("直接回车使用 5，输入数字后回车")
                .prompt()
            else {
                std::process::exit(0);
            };

            let Ok(max_count) = input.trim().parse::<usize>() else {
                eprintln!("请输入阿拉伯数字");
                continue;
            };
            let max_count = max_count.clamp(0, 5); // 限制最大20，避免过大
            seed_objects.push(Seed::new(name, price, max_count));
            break;
        }
    }

    seed_objects.extend(
        FREE_SEEDS
            .into_iter()
            .map(|(name, price)| Seed::new(name, price, 5)),
    );

    // 按价格降序排序
    seed_objects.sort_by_key(|s| Reverse(s.price));

    println!("\n正在计算最优方案...\n");

    if let Some(result) = solve(&seed_objects) {
        println!("=== 最优方案 ===");
        println!();
        println!("总价: {}", format_money(result.price));
        println!("合成花费: {}", format_money(result.cost as _));
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
