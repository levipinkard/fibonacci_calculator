use std::io;
fn main() {
    let mut a: usize = 0;
    let mut b: usize = 1;
    let mut hold: usize;
    println!("Run count?: ");
    let mut run_limit = String::new();
    io::stdin()
        .read_line(&mut run_limit)
        .expect("INVALID INPUT");
    let run_limit: u16 = run_limit.trim().parse().expect("PLEASE TYPE A NUMBER");
    let mut run_count = 0;
    while run_count < run_limit {
        if run_count == 0 {
            println!("\n0");
            run_count = run_count + 1;
            continue;
        } else if run_count == 1 {
            println!("1");
            run_count = run_count + 1;
            continue;
        }
        match a.checked_add(b) {
            Some(c) => {
                hold = c;
            }
            None => {
                println!("Too big, overflow!");
                break;
            }
        };
        println!("{}",hold);
        a = b;
        b = hold;
        run_count = run_count + 1;
    }
}
