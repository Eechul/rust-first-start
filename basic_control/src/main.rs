fn main() {
    // if표현식
    // - 일반적 if와 else if와 다수조건
    let number = 2;
    if number > 3 { // 당연하지만 반드시 bool로 반환되야 함
        println!("first if1");
    } else if number == 2 {
        println!("first else if2");
    } else {
        println!("first else3");
    }

    // - let구문에서 if 사용하기
    let condition = true;
    let n = if condition {
        5 // block 표현식
    } else {
        6 
    };

    println!("value is {}", n);


    // 반복문과 반복
    // -loop와 함께 코드의 반복 수행
    // loop {
    //     println!("again?");
    // }

    // -while과 함께하는 조건부 반복
    let mut n2 = 5;
    while n2 != 0 {
        println!("while value is {}", n2);
        n2 = n2 - 1;
    }
    // - 콜랙션 순회
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("collection for {}", element)
    }

}
