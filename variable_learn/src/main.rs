





fn add(num1:u8,num2:u8 )->u8{
    return num1+num2;
}


fn calculate_length(s:String)->(String,usize){
    let length=s.len();

    return (s,length);

}


fn refering_data(s:&mut String){
    s.push_str(" world");
}


fn main() {



    
    

    let testing_data:String=  String::from("Wap institute");

    let (testing_data1,len)=calculate_length(testing_data);

    println!("The length of {testing_data1} is {len}");



    let mut refer_data= String::from("hello");

            refering_data(&mut refer_data);

    println!("the data of refer is {refer_data}");





    let num1:u8=12;
    let num2:u8=15;
    let result:u8=add(num1,num2);
    println!("data is {result}");
    // let mut age;

    // age=25;
    // println!("my age is {age}");

    // age=28;

    // println!("my age is {age}");


    // let name:&str="wap";
    let mut name:String= String::from("institute");
    name.push_str(" wap");
    println!("{name}");

    println!("=====================");

    // tupple 

    let emp_info:(&str,u8)=("Ramesh",50);

    let emp_name= emp_info.0;
    let emp_age=emp_info.1;

    // short way we can access

    let (employee_name,employee_age)=emp_info;

    println!("employee name {emp_name} and age is {emp_age}");
    println!("employee name {employee_name} and age is {employee_age}");


}
