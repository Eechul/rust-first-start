fn main() {
    // 기본 변수는 불편성 입니다.
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // 변수병의 접두어로 'mut'를 추가하면 변수는 가변성이 됩니다.
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // 불변성 그 자체인 상수, Rust의 상수 명명 규칙에 따라 단어를 대문자로 사용
    // const MAX_AGE: u32 = 20;
    // println!("The value of x is: {}", MAX_AGE);

    // 쉐도잉(shadowing)
    // '첫번째 변수 x가 다음에 x에 대해 쉐도우 됐다.' 라고 표현한다.
    // let 키워드를 다시 사용하면서 불변성을 나타나지 않게 할 수 있다.
    // mut 접두사와 달리 같은 이름의 변수를 사용 할 수 있다.
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 쉐도잉 변수의 타입이 달라도 허용이 된다.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of x is: {}", spaces);

}
