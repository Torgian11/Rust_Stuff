#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoe_size: u32 = 11;
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 8,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 12,
                style: String::from("Reeboks Sneaker"),
            },
            Shoe {
                size: 11,
                style: String::from("Nike Sneaker"),
            },
        ];

        let found_sizes = shoes_in_size(shoes, shoe_size);
        assert_eq!(found_sizes, vec![Shoe { size: 11, style: String::from("Nike Sneaker")}]);
    }
}


fn main() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    // Next methods
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    println!("Hello, world!");

    // Map iterator

    let v2: Vec<i32> = vec![1,2,3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    assert_eq!(v3, vec![2,3,4]);
}
