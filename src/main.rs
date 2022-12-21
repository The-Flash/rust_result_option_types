// takes a list of strings and sums them
#[derive(Debug)]
struct ConversionError;

#[derive(Debug)]
struct SummationError;

fn to_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn to_int_2(s: &str) -> Result<i32, ConversionError> {
    s.parse().map_err(|_| ConversionError)
}

fn sum_vec(num_strs: Vec<String>) -> Option<i32> {
    let mut total = 0;
    for num_str in num_strs {
        total += to_int(&num_str)?;
    }
    Some(total)
}

fn sum_vec_2(num_strs: Vec<String>) -> Result<i32, SummationError> {
    let mut total = 0;
    for num_str in num_strs {
        total += to_int_2(&num_str).map_err(|_| SummationError)?;
    }
    Ok(total)
}

fn main() {
    let sum = sum_vec(vec![String::from("2"), String::from("4")]);
    println!("{:?}", sum);

    let sum = sum_vec(vec![String::from("2"), String::from("4cs")]);
    println!("{:?}", sum);
    
    let sum = sum_vec_2(vec![String::from("2"), String::from("4")]);
    println!("{:?}", sum);

    let sum = sum_vec_2(vec![String::from("2"), String::from("4cs")]);
    println!("{:?}", sum);
}
