// ## Rust进阶
// - 所有权
// - Trait
// - 生命周期
// - Iterator
// - Substrate Runtime宏的介绍
// - 作业
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        60
    }
}



pub fn main() {
    // 枚举
    println!("Hello, world!");
    let light = TrafficLight::Red;
    println!("light is :{}", light.time());
    let five= Some(5);
    let six = five.map(|i| i + 1);
    println!("six: {:?}", six);

    let none: Option<i32> = None;
    let none_result = none.map(|i| i + 1);
    println!("none: {:?}", none_result);

    // if let语法糖
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("tree")
    }

    // 所有权
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s2: {}", s2);
    println!("s1: {}", s1);

    // 参数所有权
    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    // String 所有权会转移
    let s = String::from("hello");
    takes_ownership(s);

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }

    // 基本数据类型会隐式调用copy, 之后还可以使用
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // 返回值所有权
    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }

    // 函数给所有权
    let s1 = gives_ownership();
    println!("{}", s1);

    // 通过函数返回值 把拿走的所有权给回来
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // Reference 和 Borrowing
    // 只读引用 &
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用 &mut
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // slice
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for x in slice.iter() {
        println!("{}", x);
    }

    // iter
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("first word is: {}", word);

    // 泛型
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    println!("{:#?}", integer);
    println!("{:#?}", float);

    // 泛型函数
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let integer = [1, 2, 3, 4, 5];
    let result = largest(&integer);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    // Traits
    struct Tweet {
        author: String,
        text: String,
    }
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}, {}", self.author, self.text)
        }
    }

    pub fn notify<T: Summary>(item: &T) {
        println!("{}", item.summarize());
    }

    let tweet = Tweet {author: String::from("lxt"), text: String::from("hello")};
    notify(&tweet);

    // 生命周期
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let result = longest("hello", "world");
    println!("The longest str is: {}", result);

    // static
    let static_s: &'static str = "I have a static lifetime.";
    println!("{}", static_s);
}
