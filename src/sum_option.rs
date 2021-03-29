// 2. 实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option<u32>，溢出时返回None，可以上传代码片段，或者代码的链接；
fn sum(integer_list: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for integer in integer_list.iter() {
        if u32::MAX - integer < sum {
            return None;
        } else {
            sum += integer
        }
    }
    Some(sum)
}

pub fn print_sum(integer_list: &[u32]) {
    let result = sum(&integer_list);
    match result {
        Some(sum) => {
            println!("result is {}", sum)
        },
        None => {
            println!("result is overflow")
        },
    }
}
