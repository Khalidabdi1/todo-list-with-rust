use std::io;
fn main() {
 
     // list
 let mut list:Vec<String>=vec![];

 loop {


 //input from user
 let mut input:String =String::new();
     
println!("Enter number :");
 println!("1- Add task");
 println!("2- show tasks");
 println!("3- remove task");
 println!("4- exit");

 io::stdin()
 .read_line(&mut input).expect("error");

println!("you choose : {}",input);

let choose:i32=input.trim().parse().expect("not number");

if choose==1{
 add(&mut list);
}else if choose==2{
    show(&mut list);

}else if choose==3 {
    
    rem(& mut list);
}else if choose==4 {
    break;
}
   


 }
 

}


fn add(list:&mut Vec<String>){
//add to list
println!("ADD TASK :");
let mut task:String=String::new();

//read from user
io::stdin()
.read_line(&mut task).expect("error");

list.push(task.to_string().trim().to_string());

for (i,m) in list.iter().enumerate(){
println!("{}-{}",i,m);

}



}


fn show(list:&mut Vec<String>){

    // show the list
    println!("your list is :");
    for(i,e) in list.iter().enumerate(){
        println!("{}-{}",i,e);
    }

}

fn rem(list:&mut Vec<String>){

 //show user the list before delete
 for (i,m) in list.iter().enumerate(){
println!("{}-{}",i,m);

}   
println!("choose list number to delete the task :");
let mut choosee:String=String::new();

io::stdin()
.read_line(&mut choosee).expect("err");

let index:usize=choosee.trim().parse().expect("not number");


   
list.remove(index);
println!("the task remove from list this new list :");

for(i,e) in list.iter().enumerate(){
println!("{}-{}",i,e);
}

   


}
