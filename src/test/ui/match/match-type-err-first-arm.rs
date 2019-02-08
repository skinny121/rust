fn main() {
    let _ = test_func1(1);
    let _ = test_func2(1);
}

fn test_func1(n: i32) -> i32 {
    //~^ NOTE expected `i32` because of return type
    match n {
        12 => 'b',
        //~^ ERROR mismatched types
        //~| NOTE expected i32, found char
        _ => 42,
    }
}

fn test_func2(n: i32) -> i32 {
    let x = match n {
    //~^ NOTE `match` arms have incompatible types
        12 => 'b',
        //~^ NOTE this is found to be of type `char`
        _ => 42,
        //~^ ERROR match arms have incompatible types
        //~| NOTE expected char, found integer
        //~| NOTE expected type `char`
    };
    x
}
