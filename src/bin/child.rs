fn main() {
    println!("Hello, child!");

    // create vector to store timestamps
    let mut times = Vec::new();

    println!("Child ready to receive");

    let mut i = 0;
    loop {
        // suspend self
        let pid = unsafe { libc::getpid() };
        if unsafe { libc::kill(pid, libc::SIGSTOP) } != 0 {
            panic!("Failed to suspend child");
        }

        // store the timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        times.push(timestamp);

        // finish after 50,000 iterations
        i += 1;
        if i == 9999 {
            break;
        }
    }

    println!("Goodbye, child! (Write)");

    let mut writer = csv::Writer::from_path("rt-signal-times-child.csv").unwrap();
    writer
        .write_record(&["index", "time"])
        .expect("Failed to write to file");
    for (i, timestamp) in times.iter().enumerate() {
        writer
            .serialize((i, timestamp))
            .expect("Failed to write to file");
    }

    println!("Goodbye, child! (Done)");
}