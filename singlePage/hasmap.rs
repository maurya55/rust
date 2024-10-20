use std::collections::HashMap;


fn main() {

    let mut scores=HashMap::new();
    
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Pink"),20);
    scores.entry(String::from("Pink")).or_insert(90);
    println!("{:?}",scores);
    
    // let score=scores.get(&String::from("Blue")).copied().unwrap_or(0);
    //  println!("{score}");
    for(key,value) in scores{
        println!("{key}==> {value}");
    }
    
     


    let text="hello world wonderful world";
    let mut map=HashMap::new();
    
    for word in text.split_whitespace(){
        let count= map.entry(word).or_insert(0);
        *count+=1;
    }

println!("{map:?}");

}