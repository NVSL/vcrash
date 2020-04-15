use std::process::Command;
use std::thread;
use std::time;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn rand() -> u64 {
    let mut buf: [u8; 8] = [0u8; 8];
    let mut f = File::open("/dev/urandom").unwrap();
    f.read_exact(&mut buf).unwrap();
    u64::from_be_bytes(buf)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: {} [rand max] [prog] [args ...]", args[0]);
    } else {
        let max: u64 = args[1].parse().expect("expected a u64");
        let r = if max == 0 {
            u64::MAX
        } else {
            rand() % max
        };
        println!("crashing after {} nanoseconds ...", r);
        let _ = thread::spawn(move || {
            thread::sleep(time::Duration::from_nanos(r)); // Windows needs time!
            std::process::exit(0);
        });
        let _child = Command::new(&args[2])
            .args(&args[3..])
            .spawn().expect("failed to launch");  
    }
}