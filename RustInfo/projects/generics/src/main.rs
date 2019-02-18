mod generics;

fn main() {
    {
        use crate::generics::largest;
        println!("The basic usage of generics:");

        let number = vec![12, 33, 56, 88, 0, 2];
        let result = largest::largest(&number);
        println!("Number array:{:?}, largest number:{}", number, result);

        let char_list = vec!['a', 'z', 's', 'm', 'b'];
        let result = largest::largest(&char_list);
        println!("The char list:{:?}, largest one:{}", char_list, result);

    }

    {
        use crate::generics::point::Point;
        let two_d = Point { x: 5, y: 8 };
        let diff = Point { x: "just a test", y: 12 };
        println!("The struct of point:{:?}, {:?}", two_d, diff);
        println!("Two dimision:{}, {}", two_d.x(), two_d.y());
        let fpoint = Point{x: 3.0, y: 4.0};
        println!("vector {:?} length: {}", fpoint, fpoint.distance_from_origin());
    }
}
