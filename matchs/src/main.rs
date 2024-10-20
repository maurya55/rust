
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}


#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime
    // Quarter(UsState)
}




fn main() {

    let da= Coin::Nickel;
    // get_data(da);
// get_data(Coin::Penny);

    println!("check data {:?}",  get_data(da));


    let data= Some(1);
    check_some(data);
}

fn check_some(da:Option<i32>){

    match da{
        Some(value)=>println!("the value is {value}"),
        None=>println!("No value found."),
    }

// println!("testing");

}

fn get_data(coin:Coin)->u8{

    match coin {
        Coin::Penny=>1,
        Coin::Nickel=>12,
        Coin::Dime=>33
    }

        // println!("{:?}",co);
}