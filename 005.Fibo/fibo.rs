// 피보나치 수열 30번째 항까지 출력하기

fn main() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", b);
    println!("{}", a);
    for i in 2..31 {
        let temp = a;
        a += b;
        b = temp;
        println!("{}", a);
    }
}