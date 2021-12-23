// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    const EVERYTHING: u32 = 42;
    call_me(EVERYTHING);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
