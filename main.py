from dataclasses import dataclass


@dataclass(frozen=True)
class Seed:
    name: str
    price: int
    count: int = 1


SEEDS = [
    # 排除至臻：必定不划算
    # Seed("蟠桃", 24_000_000, 5),
    # Seed("大王菊", 6_000_000, 5),
    # Seed("松果", 4_000_000, 5),
    # Seed("幻月花", 10_000_000, 5),
    # Seed("红包树", 8_880_000, 5),
    # 普通种子
    Seed("草莓", 1_250_000, 5),
    Seed("向日葵", 3_000_000, 5),
    Seed("猕猴桃", 600_000, 5),
    Seed("香蕉", 300_000, 5),
    Seed("苹果", 150_000, 5),
    Seed("玉米", 90_000, 5),
    Seed("西瓜", 60_000, 5),
    Seed("竹子", 30_000, 5),
    Seed("黄瓜", 9_000, 5),
    Seed("波斯菊", 2_000, 5),
    Seed("番茄", 100, 5),
    Seed("土豆", 0, 5),
    # 月球种子
    Seed("液光藤", 3_100_000, 5),
    Seed("月核树", 2_000_000, 5),
    Seed("星叶菜", 500_000, 5),
    Seed("月莓", 300_000, 5),
    Seed("银灰苔", 120_000, 5),
    Seed("月环树", 60_000, 5),
    Seed("月番茄", 16_000, 5),
    Seed("月灯草", 8_000, 5),
    Seed("灰壤豆", 1_000, 5),
    Seed("月光草", 0, 5),
]


def reward_level(total_price: int) -> tuple[int, int]:
    """
    返回:
    (收益概率, 能量增长)
    """

    if total_price > 1_200_000:
        return 25, 4

    if total_price > 500_000:
        return 5, 3

    if total_price > 25_000:
        return 1, 2

    if total_price > 0:
        return 0, 1

    return 0, 0


def expand_inventory(seeds):
    expanded = []

    for seed in seeds:
        expanded.extend([seed] * seed.count)

    return expanded


def solve(seeds):
    SCALE = 100

    items = expand_inventory(seeds)

    prices = [s.price // SCALE for s in items]

    max_sum = sum(prices)

    # dp[k][sum] = 是否可达
    dp = [[False] * (max_sum + 1) for _ in range(6)]
    parent = [[None] * (max_sum + 1) for _ in range(6)]

    dp[0][0] = True

    for idx, price in enumerate(prices):
        for k in range(4, -1, -1):
            for s in range(max_sum - price + 1):
                if not dp[k][s]:
                    continue

                ns = s + price

                if not dp[k + 1][ns]:
                    dp[k + 1][ns] = True
                    parent[k + 1][ns] = (k, s, idx)

    best = None

    for k in range(1, 6):
        for s in range(max_sum + 1):
            if not dp[k][s]:
                continue

            total_price = s * SCALE

            probability, energy = reward_level(total_price)

            # 收益优先，总价越低越好
            score = (
                probability,
                energy,
                -total_price,
            )

            if best is None or score > best[0]:
                best = (
                    score,
                    k,
                    s,
                )

    if best is None:
        return None

    _, k, s = best

    chosen_items = []

    while k > 0:
        pk, ps, idx = parent[k][s]

        chosen_items.append(items[idx])

        k = pk
        s = ps

    chosen_items.reverse()

    counts = {}

    for seed in chosen_items:
        counts[seed.name] = counts.get(seed.name, 0) + 1

    total_price = sum(seed.price for seed in chosen_items)

    probability, energy = reward_level(total_price)

    return {
        "price": total_price,
        "probability": probability,
        "energy": energy,
        "counts": counts,
        "chosen": chosen_items,
    }


def format_money(x: int) -> str:
    return f"{x:,}"


def main():
    result = solve(SEEDS)

    if result is None:
        print("无可行方案")
        return

    print("=== 最优方案 ===")
    print()

    print(f"总价: {format_money(result['price'])}")
    print(f"出金概率: +{result['probability']}%")
    print(f"能量增长: +{result['energy']}")

    print()
    print("选择种子:")

    for name, count in sorted(result["counts"].items()):
        print(f"  {name:<8} x{count}")

    print()
    print("详细列表:")

    for seed in result["chosen"]:
        print(f"  {seed.name:<8}{format_money(seed.price)}")


if __name__ == "__main__":
    main()
