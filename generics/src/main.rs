use generics::*;

fn main() {
    let arr = [1, 2, 5];
	println!("{}", identity("Hello, world!"));
	println!("{}", identity(3));
	println!("{}", identity(3.5));
	println!("{:?}", identity(arr));
}

/* 
Hello, world!
3
*/