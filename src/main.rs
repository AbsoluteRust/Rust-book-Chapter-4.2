fn main() 
{
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    let mut s = String::from("hello");

    change(&mut s);

    {//curly brackets required to 
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


    println!("The length of '{}' is {}.", s1, len);
    println!("{}", s);

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize // & lets you reference without taking ownership, the opposite would be * (dereferencing). s is a reference to a String
{
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.


  fn change(some_string: &mut String) { //has to explicitly br **mutable** here, cannot borrow a mutable twice, as it must be unborrowed 
    some_string.push_str(", world");
}  

fn dangle() -> String { // dangle returns a reference to a string, instead this is fixed by returning the STring directly (not a reference) 
  let s = String::from("hello"); //s is new string

  s //return a reference to the String, "s", fix this by making it a string instead of reference to string
}//s goes out of scope, and is dropped. its memory goes away


//lessons learned:
//at any given time, you can have *either* one mutable ref, *or* any number of immutable refs
//references must always be valid