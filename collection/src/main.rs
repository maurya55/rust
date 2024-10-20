

#[derive(Debug)]
enum SpredSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}


fn main() {

    let vec: Vec<SpredSheetCell>= vec![SpredSheetCell::Int(12),SpredSheetCell::Float(12.2),SpredSheetCell::Text(String::from("wap"))];
    println!("{:?}",vec);

  match vec.get(0){
        Some(SpredSheetCell::Int(value)) => println!("value of first is {value}"),
        Some(value)=>{
            println!("value of other data is {:?}",value)
        },
        None => {
            println!("value of not found")
        },
    }


    // let mut vec= Vec::new();
    // let mut vec= vec![1,2,3];

    // vec.push(1);
    // vec.push(12);

    // let fouth_value=&vec[1];

    // println!("{:?}",fouth_value);


    // let mut vec= vec![1,2,3];
    // let fourth_value = match vec.get(21) {
    //    Some(value)=>value,
    //    None=>{
    //     &-1
    //    }
    // };
    // println!("{fourth_value}");


    // let fouth_value =vec.get(1);

    //  if  fouth_value==None{
    //     println!("value not found");
    // }
    // else{
    //     println!("{:?}",fouth_value);

    // }

    // println!("{:?}",vec);
    // println!("{:?}",fouth_value);

}
