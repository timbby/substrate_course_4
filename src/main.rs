mod test;
mod traffic_light;
mod sum_option;
mod calculate;

use traffic_light::{print_time, TrafficLight};
use sum_option::print_sum;
use calculate::{calculate_area, Triangle, RoundShape};

fn main() {
    // 上课内容
    test::main();

    // 1. 为枚举交通信号灯实现一个trait，trait里包含一个返回时间的方法，不同的等持续的时间不同，可以上传代码片段，后者代码的链接；
    print_time(&TrafficLight::Red);
    print_time(&TrafficLight::Green);
    print_time(&TrafficLight::Yellow);

    // 2. 实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option<u32>，溢出时返回None，可以上传代码片段，或者代码的链接；
    print_sum(&[1, 2, 3, 4]);
    print_sum(&[1, 2, 3, 4294967295]);

    // 3. 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，需要用到泛型和泛型约束，可以上传代码片段，或者代码的链接。
    calculate_area(&Triangle::create(3.0, 4.0, 5.0));
    calculate_area(&Triangle::create(5.0, 12.0, 13.0));
    calculate_area(&RoundShape::create(1.0));
    calculate_area(&RoundShape::create(5.0));
}
