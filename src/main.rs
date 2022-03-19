fn main() 
{
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize // & lets you reference without taking ownership, the opposite would be * (dereferencing). s is a reference to a String
{
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.


//finish this later, https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing