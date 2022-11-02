// Author: yqq
// Date: 2022-11-02 23:06:06
// Description: 指定一种格式输出

use std::fmt;

struct City {
    name: &'static str,
    // 纬度
    lat: f32,
    // 经度
    lon: f32
}

impl fmt::Display for City {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_xx = if self.lat > 0.0 { "N" } else {"S"};
        let lon_xx = if self.lon > 0.0 { "E" } else {"W"};

        write!(f, "{}, {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_xx, self.lon.abs(), lon_xx)
    }
}




#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue )
    }
}

fn main() {

    println!("{}", City{name: "Beijing", lat:43.2342, lon: -123.23});

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }



    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{:?}", *color);
    }



    println!("===============");
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        // println!("{:?}", *color);
        println!("{}", *color);
    }

}