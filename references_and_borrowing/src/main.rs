fn main() {
    // 참조자와 빌림
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("1. The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2); // 가변 참조자 이용
    println!("2. '{}' ", s2);

}

fn calculate_length(s: &String) -> usize { // s는 String의 참조자, 이렇게 만든것을 빌림 이라고 부른다.
    // s.push_str(", world"); // 빌린 값을 고쳐보려하면 에러가 발생
    s.len()
} // 직접적인 소유권이 없기 때문에 메모리를 반납하는 일은 일어나지 않는다.

fn change(s: &mut String) { // 가변 참조자를 이용하면 값을 고칠 수 있다.
    s.push_str(", world");
}
