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