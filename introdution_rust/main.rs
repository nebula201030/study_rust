/*
//출력을 해보자
fn main()
{
    let mut x = 5;
    println!("the value of x is: {}",x);
    x=6;
    println!("the value of x is: {}",x);

    const MAX_POINTS: u32 = 100_000;

    println!("the Max_Points is {}",MAX_POINTS);
}
*/
/*
//함수 만들기
fn main()
{
    another_function(12, 7);
}

fn another_function(x: i32,y:i32)
{
    println!("the value of x is {}", x);
    println!("the value of x is {}", y);
}
*/
// fn main()
// {

//     let y = 
//     {
//         let x = 3;
//         x + 1
//     };
//     println!("the value of x is {}",y);
// }

// 반환값을 갖는 함수
// fn five() -> i32
// {
//     5
//     //함수 호출, 매크로 let도 없는데 5라는 숫자 하나로 함수가 허용됨 함수 반환 
//     //값이 i32인데 이게 뭔지 궁금함

// }

// fn main()
// {
//     let x = five();
//     println!("the value of x is: {}" ,x);
// }
// fn main()
// {
//     let x = plus_one(5);
//     println!("the value of x is {}",x);
// }

// fn plus_one(x: i32) -> i32
// {
//     x + 1
//     // x+1; 이라는 문법일것 같지만 x+1을 할때 작동한다 오히려 ;을 붙이면
//     //오류가 발생하는 것이 보인다.
//     //lismatched types 라는 오류로 plus_one 함수의 정의는 i32값을 반환
//     //한다는 것이다 하지만 구문은 값을 산출하지 않기에 ()처럼 비어있는
//     //튜플로 표현된다
// }
// fn main()
// {
//     let num = 3;
//     if num < 5
//     {
//         println!("comdition was true");
//     }
//     else 
//     {
//         println!("condition was false");
//     }
// }

// fn main()
// {
//     let num = 3;
//     if num
//     {
//         println!("num is Three");
//     }
//     /*
//     러스트는 ruby, js와 다르게 boolean이 아닌 타입을 자동으로 boolean으로
//     바꾸지 않는다 
//     */
// }

// fn main()
// {
//     let condition = true;
//     let number = if condition
//     {
//         5
//     }
//     else
//     {
//         6
//     };
//         println!("the value of number is:{}",number);
    
// }

// fn main()
// {
//     loop
//     {
//         println! ("again");
//         //무한 반목
//     }
// }
// fn main()
// {
//     let mut number = 3;

//     while number != 0
//     {
//         println!("{}",number);
//         number = number -1;
//     }
//     println!("liftoFF");
// }
// fn main()
// {
//     let a = [10,20,30,40,50];
//     met mut index = 0;

//     while index<5
//     {
//         println!("the value is:{}",a[index]);
//         index = index+1;
//     }
// }
// fn main()
// {
//     let a = [10,20,30,40,50];
//     for elemet is a.iter()
//     // iter이란?
//     {
//         println!("the value is:{}",elemet);
//     }
// }

// fn main()
// {
//     for number in (1..10).rev()
//     // 일단 공부는 해보겠지만 (1..10 )이라는 것이 1~ 10까지 인듯 ㅇㅇ
//     {
//         println!("{}!",number);
//     }
//     println!("LIFTOFF");
// }
/*
소유권 
러스트의 핵심 기능은 바로 소유권이다 
모든 프로그램은 실행하는 동안 컴퓨터의 메모리를 사용하는 방법을
관리해야 된다 
몇몇 언어들은 프로그램이 실행되는 동아에 사용하지 않는 메모리가 끊임없이 찾는 
가비지 콜렉션을 갖고 있다 다른 언어들이 프로그래머가 직접
명시적으로 메모리를 할당하고 해제해야 된다 러스트는 제 3의 접근법을 사용한다
메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성되 소유권 시스템을
통합 관리합니다 소유권 기능들의 어떤 것도 런타임 비용을 발생하지 않는다.
소유권이란 개념이 많은 프로그래머들에게 새로운 것이기 때문에 이해하고 사용하는데
시간이 걸린다  러스트의 소유권 시스템의 규칙에 더 많은 경험을 할 수록 더욱
안전하고 효율적인 코드를 개발할 수 있게된다.

스택과 힙
많은 프로그램 언어들은 스택과 힙을 자주 생각할 필요가 없다 하지만
러스트와 같은 시스템 프로그래밍 언어에서는 값이, 수택에 있는지 힙에 있는지
에 대한 여부에 따라서 언어의 동작과 방식에 큰 영향을 준다

소유권 규칙
러스트의 각각의 값은 해당값의 오너라고 불리는 변수를 갖는다
한번에 하나의 오너만이 존재한다
오너가 스코프 밖으로 벗어나는 때 값이 버려진다
*/

// fn main()
// {
//     let mut s =String :: from("hello");
//     //기본으로 string이 되어 있음
//     s.push_str(", world");
//     println!("{}",s);
// }

// fn main()
// {
//     let stirng_first = String::from("hello");
//     let string_second = stirng_first.clone();

//     println!("string_first = {}, string_second = {}",string_first, string_second);

// }

// fn main() {
// let s1 = String::from("hello");
// let s2 = s1.clone();

// println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn takes_ownership(some_string: String)
// {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32)
// {
//     println!("{}", some_integer);
// }

// fn main()
// {
//     let s = String::from("hello");

//     takes_ownership(s);

//     let x = 5;

//     makes_copy(x);
// }

// fn gives_ownership() -> String
// {
//     let some_string = String::from("hello");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String
// {
//     a_string
// }

// fn main()
// {
//     let string_first = gives_ownership();

//     let string_second = String::from("hello");

//     let string_third = takes_and_gives_back(string_second);
// }
// // 변수의 소유권은 모든 순간 똑같은 패턴을 따른다.
// //모든 함수가 소유권을 가져싿가 반납한다

// fn calculate_length(s: String) -> (String, usize)
// {
//     let length = s.len();
//     (s, length)
// }

// fn main()
// {
//     let string_first = String::from("hellwo");

//     let (string_second, len) = calculate_length(string_first);

//     println!("the length of '{}' is {}",string_second, len);
// }
// fn calculate_length (s: &String) -> usize
// {
//     s. len()
// }

// fn main()
// {
//     let stirng_first = String::from("hello world");

//     let len = calculate_length(&stirng_first);

//     println!("the length of '{}' is {}", stirng_first,len);
// }

// fn change(some_string: &mut String)
// {
//     some_string.push_str(", world");
// }

// fn main()
// {
//     let mut s = String::from("hello");

//     change(&mut s);
// }
/*
참조자의 규칙
1. 어떠한 경우에도 아래에 둘중 하나만 가질 수 있다
 히니의 가변 참조자
  임의 개수의 불변 참조자

2. 참조자는 항상 유효해야 된다
*/
// fn first_word(s: &String) -> usize
// {
//     let bytes = s.as_bytes();

//     for(i,&item) in bytes.iter().enumerate()
//     {
//         if item == b ''
//         {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main()
// {
//     let mut a = String::from("hello world");

//     let word = first_word(&s);

//     s.clear();
// }

// struct User
// {
//     username:String,
//     eamil: String,
//     sign_in_count: u64,
//     active: bool,
// }

// let user1 = User
// {
//     email: String::from("nebula201030@gmail.com"),
//     username: String::from("nebula201030"),
//     active: true,
//     sign_in_count: 1,
// };

// let user2 = User
// {
//     email: String::from(""),
//     username: String::from(""),
//     ..user1
// };

// user1.email= String::from("godos0104@gmail.com")

// fn build_user(email: String, username: String) -> User
// {
//     User
//     {
//         email:email,
//         username:username,
//         active:true,
//         sign_in_count: 1,
//     }
// }

// fn main()
// {
//     let length1 = 50;
//     let width1 = 30;

//     println!
//     ("the area of the rectangle is {} square pixels.", area(length1, width1));
// }

// fn area(length:u32, width: u32) -> u32
// {
//     length * width
// }

// fn main(){
//     let rect1 = (50,30);

//     println!(
//         "the area of the rectangle is {} square pixels.", area(rect1)
//     );
// }

// fn area(dimensios: (u32, u32)) -> u32{
//     dimensios.0 * dimensios.1
// }

// struct Rectangle
// {
//     length : u32,
//     width: u32,
// }

// fn main()
// {
//     let rect1 = Rectangle{ length: 500, width: 30};

//     println!
//     (
//     "the area of the rectangle is {} square pixels.", area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32
// {
//     rectangle.length * rectangle.width
// }
// struct Person{
//     name:String,
//     age: u8,
//     likes_oranges: bool,
// }

// //struct Point2D(u32,u32);

// //struct Unit;

// fn main(){
//     let person = Person{
//         name: String::from("Adam"),
//         likes_oranges: true,
//         age: 25
//     };

//     //let origin = Point2D(0,0);

//     //let unit = Unit;

//     if person.likes_oranges{
//         println!("{:?} is {:?} and likes orange",person.name, person.age);      
//     }
//     else{
//         println!("{:?} is {:?} and doesn't like orange",person.name,person.age);
//     }
// }
