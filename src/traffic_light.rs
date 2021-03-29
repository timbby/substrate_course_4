// 1. 为枚举交通信号灯实现一个trait，trait里包含一个返回时间的方法，不同的等持续的时间不同，可以上传代码片段，后者代码的链接；
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait TrafficLightTime {
    fn time(&self) -> u8;
}

impl TrafficLightTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red {} => {
                60
            },
            TrafficLight::Green {} => {
                15
            },
            TrafficLight::Yellow {} => {
                5
            },
        }
    }
}

pub fn print_time<T: TrafficLightTime>(item: &T) {
    println!("traffic light time is: {}", item.time());
}