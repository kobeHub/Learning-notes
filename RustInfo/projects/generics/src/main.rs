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

        let x = "Just a str literal";
        let y = "Another thing";
        println!("str:{}, {}, longest:{}", x, y, largest::str_longest(x, y));

        let result;
        let str1 = String::from("A long string");
        {
            let short = "live long enough, it's amazing";
            result = largest::str_longest(str1.as_str(), short);
        }
        println!("&str lives long enough to a borrow, {}", result);

        println!("Now try to find the first world of a sentence:");
        let sentence = "Just a sentence";
        println!("{}, and first world {}", sentence, largest::first_word(sentence));
    }

    {
        use crate::generics::point::Point;
        let two_d = Point { x: 5, y: 8 };
        let diff = Point { x: "just a test", y: 12 };
        println!("The struct of point:{:?}, {:?}", two_d, diff);
        println!("Two dimision:{}, {}", two_d.x(), two_d.y());
        let fpoint = Point{x: 3.0, y: 4.0};
        println!("vector {:?} length: {}", fpoint, fpoint.distance_from_origin());
        let mix = diff.mixup(fpoint);
        println!("After mixup: {:?}", mix);
    }

    {
        use crate::generics::traits;

        let numbers = vec![12.0, 8.0, 5.5, 88., 99.1];
        let chars = vec!['a', '+', '[', 's'];
        let total = traits::total_length(&numbers, &chars);
        println!("The total length {}", total);
        println!("The longer str: {}", traits::longest_with_anno("A long str", "as", "Let's go deep into rust"));
    }

    {
        use crate::generics::exception::ImportantExcept;
        println!();
        let novel = String::from("It's an ancient story;I will tell u in detail");
        /*let name = novel.split(';')
            .next()
            .expect("Couldn't find a ;");

        let con = novel.split(';')
            .next()
            .expect("Could not find a ;");*/
        let contents: Vec<_> = novel.split(';')
            .collect();
        let mut i = ImportantExcept{
            name: contents[0],
            except: contents[1],
        };
        println!("Got an exception:{:#?}", i);
        println!("The level of the exception: {}", i.level());
        println!("The except message: {}", i.except_and_announce("!!Caution!!"));

        i.multi_args(";;Caution;;", "another exception");
    }
}
