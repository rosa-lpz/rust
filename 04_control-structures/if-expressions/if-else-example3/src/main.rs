fn main() {
    let hours:f64 = 45.0;
	let rate:f64=10.5;
    
    if hours > 40.0 {
    	let hours_above:f64= hours-40.0;
    	let pay:f64=(40.0*rate) + hours_above*(rate*1.5);
        println!("Pay is {pay}");
    } else{
        let pay:f64=hours*rate;
        println!("Pay is {pay}");
    }
	
}