//fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}
//
//fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//    x
//}

//fn main() {
//    let string1 = String::from("long string is long");
//    let result;
//
//    {
//        let string2 = String::from("xyz");
//        result = longest(string1.as_str(), string2.as_str());
//    }
//    println!("The longest string is {}", result);
//}
//
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn print_excerpt(&self) {
        println!("{}", self.part)
    }
}

fn main() {
    let novel = String::from("Call me Ninan. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not split");

    let i = ImportantExcerpt { part: first_sentence };

    i.print_excerpt()
}
