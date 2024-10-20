
struct User {
    username: String,
    email: String,
}

struct Post(u8,u8,u8);

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn cal_area(&self)->u32{
        self.height*self.width
    }

    fn can_hold(&self, other: &Rectangle )->bool {
        self.width>other.width && self.height>other.height
    }

    fn square(side:u32)->Rectangle{
        // fn square(side:u32)->Self{
        Rectangle{
            width:side,
            height:side
        }
    }
}

fn calculate_are(react: &Rectangle)->u32 {

    react.width*react.height
}


fn main() {

let react = Rectangle{
    width:12,height:12
};
let react2 = Rectangle{
    width:13,height:12
};

 let area= calculate_are(&react);
 println!("the are of rectangle is {}, full data is {:?} ",area, react);
 println!("calculate are with method {:?} ", react.cal_area());

 println!("can react hold react 2 {}",react.can_hold(&react2));


 let sq=Rectangle::square(5);
    println!("square data {sq:?}");

//  println!("the are of rectangle is {area}, full data is {react:#?} ");


 

let postdata :Post =Post(12,14,15);
     post_data(postdata);


    let mut ab= User {
        username: String::from("wap"),
        email:String::from("wap@gmail.com"),
    };

    ab.username=String::from("Wap ");
    ab.username.push_str("institute");
    // let ab = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };
    println!("{}",ab.username);
}


fn post_data(data: Post){
    println!("{}",data.0);
}