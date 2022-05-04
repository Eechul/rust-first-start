fn main() {
    // 1.스칼라 타입들
    // - 정수형
    // 길이 (8-bit, 16-bit, 32-bit, 64-bit, arch[커스텀?]), 
    // 부호 있는, 부호가 없는 에 따라 접두사에 i 혹은 u가 붙는다.
    // 예: i8(부호가 있는 8비트 정수), u8(부호가 없는 8비트 정수)
    let vari1:i8 = -2;
    // let vari2:u8 = -2; // 컴파일 에러
    let vari2:u8 = 2; // 컴파일 성공
    println!("{} {}", vari1, vari2);

    // - 부동 소수점 타입
    // f34, f64 접미사에 숫자는 비트 크기
    let vari3 = 3.2; // default f64
    let vari4:f32 = 2.0;
    println!("{} {}", vari3, vari4);

    // - 수학적 연산들[ +, -, *, /, % ]
    // 예제 생략

    // - Boolean 타입
    let vari5 = true;
    let vari6: bool = false; // bool로 사용
    println!("{} {}", vari5, vari6);

    // - 문자 타입, 작은따옴표를 씀. 뒤에 나올 String은 큰따옴표 사용
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    // 2. 복합 타입 [튜플과 배열]
    // - 값들을 집합시켜 튜플화하기
    let tup1: (i32, f64, u8) = (-2, 2.2, 10);
    let (x, y, z) = tup1;
    println!("구조해체 x y z : {} {} {}", x, y, z);

    // 튜플을 색인으로 접근할 수 있다.
    let tup_first = tup1.0;
    let tup_second = tup1.1;
    let tup_third = tup1.2;
    println!("구조해체 x y z : {} {} {}", x, y, z);

    // - 배열, 튜플과 다르게 모든 값의 타입이 같아야 한다.
    // 가변적이지 않은 타입이다. 배열과 비슷하면서 가변적인 타입은 벡터가 있다. 확장, 축소가 용이하다. 뒤에 나올것
    let arr1 = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // 배열 요소 접근
    let arr_first = arr1[0];
    let arr_second = arr1[1];
    let arr_third = arr1[2];
    // let arr_error = arr1[10]; // error
    println!("{} {} {}", arr_first, arr_second, arr_third);

}