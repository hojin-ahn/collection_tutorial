fn main() {
    // let v: Vec<i32> = Vec::new();
    //
    //
    // let v = vec![1, 2, 3];
    //
    //
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    //
    //
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }
    //
    //
    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
    //
    //
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
    //
    //
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }
    //
    //
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    //
    //
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];
    //
    //
    // let mut s = String::from("foo");
    // s.push_str("bar");
    //
    //
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");
    //
    //
    // let mut s = String::from("lo");
    // s.push('l');
    //
    //
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // // let s3 = s1 + &s2;
    // let _s = format!("{s1}-{s2}");
    //
    //
    // for c in "3월".chars() {
    //     println!("{c}");
    // }
    // for b in "3월".bytes() {
    //     println!("{b}");
    // }
    //
    //
    // use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    //
    //
    use std::collections::HashMap;
    let text = "hello world beautiful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
