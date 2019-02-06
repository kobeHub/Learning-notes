// slice test
//pub fn slice_test() {
//    let s = "Just a test".to_string();
//    let sl1 = &s[..];
//    let sl2 = &s[6..];
//    let () = sl2;
//}

// get the first world of string
pub fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
