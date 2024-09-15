fn find_largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// This throws an error:  consider restricting type parameter `T`
// : std::cmp::PartialOrd
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// Point structure with T and U types to allow for mixing of integers and floats
// struct Point<T, U> {
//     x: T,
//     y: U,
// }


// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }

//     fn y(&self) -> &T {
//         &self.y
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}


// monomorphixation examples using Options

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}


fn main() {
    let number_list = vec![34,50,110,35,72];
    let result = find_largest_number(&number_list);

    println!("The largest number in the list is {result}");

    let char_list = vec!['y', 'm', 'a', 'v'];
    let char_result = find_largest_char(&char_list);
    println!("The largest character in the list is {char_result}");

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 5.5, y: 4.3};
    // let integer_and_float = Point {x: 2, y: 2.2};

    // let p = Point { x: 5.2, y: 1.02 };

    // println!("p.x = {}", p.x());
    // println!("p.y = {}", p.y());

    // let p_f = Point {x: 5.4, y: 10.33 };
    // println!("distance from origin = {}", p_f.distance_from_origin());

    // X1 Y1 X2 Y2 mixup example, generic types different from its structure definition
    let p1 = Point {x: 5, y: 10.2};
    let p2 = Point {x: "Hello!", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //monomorphization example using option_i32 / f64
    let integer_1 = Option_i32::Some(5);
    let float_1 = Option_f64::Some(5.5);

}

