fn main() {
    // 함수 동작 원리
    // 기본적인 함수
    another_function1(5, 10);

    let x = 5;
    let y = {
        // block {}은 표현식이 된다.
        let x = 3; // 위 x와 겹치지 않음
        x + 1 // 표현식 종결은 세미콜론을 쓰지 않는다.
    };


    let fifteen = another_function2(5, 10);
    println!("The value of x + y is {}", fifteen);
}

fn another_function1(x: i32, y: i32) { // 매개변수 x,y는 32비트 정수
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn another_function2(x: i32, y: i32) -> i32 { // 반환값의 타입을 명시할 수 있음
    x + y // 표현식 종결은 세미콜론을 쓰지 않는다. 라는 문법을 상기
}