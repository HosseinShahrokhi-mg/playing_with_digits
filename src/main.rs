use std::convert::TryInto;
fn main() {
    dig_pow();
}



fn dig_pow(n: i64,mut p: i32) -> i64 {
   let mut sum_pow: i64 = 0;
    let digits = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>();
    
    println!("{:?}", digits);
    
    for i in digits {
        sum_pow = sum_pow + i.pow(p.try_into().unwrap()) as i64;
        p += 1;
    }
    let k: i64;
    k = (sum_pow/n).try_into().unwrap();
    if (n*k) == sum_pow {
        k
    }else{
        -1
    }
    
}
