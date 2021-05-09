fn main() {
    let mut var: i8 = 6;
    println!("The value of the variable is {}",var);
    var = 5;
    println!("The value of the variable is {}",var);

    let overflow_examples:u8 = 255;
    println!("The value of the variable is {}",overflow_examples);
    println!("An overflow has occurred {:?}", overflow_examples.overflowing_add(1));
    println!("Overflowed with wrapping 255 + 1= {}", overflow_examples.wrapping_add(1));
    println!("None if overflow occurred {:?}", overflow_examples.checked_add(1));
    println!("Maximum if overflow occurs {}", overflow_examples.saturating_add(1));

    let test_float: f64 = 1.12;
    println!("The value of the variable is {}",test_float);

    let tup: (&str, u8,i8,char) = ("test",1,-1,'z');
    let (x,y,z,a) = tup;
    let second_element = tup.1;

    let ar:[u8;3] = [1,2,3];
    let ar_2:[u8;3]=[1;3];
}
