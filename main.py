from dataclasses import dataclass


@dataclass(frozen=True)
class Seed:
    name: str
    price: int


SEEDS = [
    # 普通种子
    Seed("蟠桃", 24_000_000),
    Seed("大王菊", 6_000_000),
    Seed("松果", 4_000_000),
    Seed("草莓", 1_250_000),
    Seed("向日葵", 3_000_000),
    Seed("猕猴桃", 600_000),
    Seed("香蕉", 300_000),
    Seed("苹果", 150_000),
    Seed("玉米", 90_000),
    Seed("西瓜", 60_000),
    Seed("竹子", 30_000),
    Seed("黄瓜", 9_000),
    Seed("波斯菊", 2_000),
    Seed("番茄", 100),
    Seed("土豆", 0),

    # 月球种子
    Seed("幻月花", 10_000_000),
    Seed("红包树", 8_880_000),
    Seed("液光藤", 3_100_000),
    Seed("月核树", 2_000_000),
    Seed("星叶菜", 500_000),
    Seed("月莓", 300_000),
    Seed("银灰苔", 120_000),
    Seed("月环树", 60_000),
    Seed("月番茄", 16_000),
    Seed("月灯草", 8_000),
    Seed("灰壤豆", 1_000),
    Seed("月光草", 0),
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


def solve(seeds):
    SCALE = 100

    prices = [s.price // SCALE for s in seeds]

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

    _, k, s = best

    chosen_indices = []

    while k > 0:
        pk, ps, idx = parent[k][s]
        chosen_indices.append(idx)
        k = pk
        s = ps

    chosen_indices.reverse()

    chosen = [seeds[i] for i in chosen_indices]

    total_price = sum(seed.price for seed in chosen)

    probability, energy = reward_level(total_price)

    return {
        "price": total_price,
        "probability": probability,
        "energy": energy,
        "seeds": chosen,
    }


def format_money(x: int) -> str:
    return f"{x:,}"


def main():
    result = solve(SEEDS)

    print("=== 最优方案 ===")
    print()

    print(f"总价: {format_money(result['price'])}")
    print(f"出金概率: +{result['probability']}%")
    print(f"能量增长: +{result['energy']}")

    print()
    print("选择种子:")

    for seed in result["seeds"]:
        print(
            f"  {seed.name:<8} "
            f"{format_money(seed.price)}"
        )


if __name__ == "__main__":
    main()