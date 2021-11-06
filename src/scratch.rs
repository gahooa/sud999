

use crate::time;

struct Point {
    x: f64,
    y: f64,
}

pub fn run(){


    let l2 = [1.2,2.3,2.4,3.5,2.3];

    let mut vec:Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);

    for n in 0..5 {
        for v in &vec {
            println!("Hi vec {}", v);
        }
        vec.push(n);
    }

    for i in 10..20 {
        let p = Point{ x: i as f64, y: 20.0 };
        println!("Hello, world! {1} {0}", p.x, p.y);
    }

    while let Option::Some(v) = vec.pop() {
        println!("popping {}", v);
    }

    for i in l2 {
        println!("over array l2 {}", i);
    }


    for i in 0..1000001 {
        if i % 100000 == 0 {
            println!("{:?}", time::unix_timestamp_f64());
        }
    }



}