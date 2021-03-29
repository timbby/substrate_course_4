// 3. 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，需要用到泛型和泛型约束，可以上传代码片段，或者代码的链接。
pub trait Graph {
    fn area(&self) -> f32;
}

// 三角形 triangle
pub struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

// 圆形 Round shape
pub struct RoundShape {
    r: f32,
}

impl Graph for Triangle {
    fn area(&self) -> f32 {
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).powf(0.5)
    }
}

impl Triangle {
    pub fn create(a: f32, b: f32, c: f32) -> Triangle{
        Triangle {
            a,
            b,
            c,
        }
    }
}

impl Graph for RoundShape {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.r.powf(2.0)
    }
}

impl RoundShape {
    pub fn create(r: f32) -> RoundShape{
        RoundShape {
            r,
        }
    }
}

pub fn calculate_area<T: Graph>(graph: &T) {
    println!("{}", format!("graph area is: {}", graph.area()))
}
