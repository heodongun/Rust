// fn main() {
//     let a:i64=1;
//     a=2;
//     println!("{}",a);
// }
//이건 안됨 왜냐면 이건 불변성 변수에 재할당을 하는것이기 때문에 안됨.

// fn main(){
//     let mut _x=1;
//     _x=2;
//     println!("{}",_x);
// }이거 _x이거는 읽히지 않을 변수는 이렇게 쓰는것이다.사용되지 않는 값

//Shadowing
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is: {}", x);
// }러스트에서 변수의 shadowing을 사용하면 새로운 값을 할당하면서 기존 값을 덮어쓸 수 있습니다. 
//이는 불변 변수(let)에도 적용되며, 매번 새로 선언하는 것처럼 동작합니다.

//Mutability
// fn main(){
//     let mut x=10;
//     x=11;
//     println!("{}",x);
// }
// _x와 같은 변수 이름은 미사용 변수를 나타내기 위해 사용됩니다.
// 가변성(mutable)과 _는 서로 다른 개념이므로, 가변성을 나타내는 mut와는 관련이 없습니다.
// 국룰이라기보다는 러스트에서 코드의 의도를 명확히 나타내기 위한 규칙입니다.

// fn main(){
//     let name=String::from("허동운");
//     println!("{}",name.len());
// }
//.len()이렇게 해주면 문자열의 길이를 반환해준다.

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     for i in tup{
//         println!("{}",i);
//     }
// }슬픈 사실 튜플은 반복 가능한 자료형이 아니므로 for 루프에 바로 사용할 수 없습니다.

fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1; // 가변 참조
    s1.push_str(", world!"); // 가변 참조를 통해 값을 변경할 수 있음
    println!("{}", s1);
}

