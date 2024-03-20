// 거스름돈 계산

fn main() {
    let mut money = 5730;
    let arr = [5000, 1000, 500, 100, 50, 10];
    for i in arr {
        println!("{:4}원 {}개", i, money / i);
        money %= i;
    }
}