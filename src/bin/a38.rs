// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let hello = std::thread::spawn(msg_hello);
    let thread = std::thread::spawn(msg_thread);
    let excited = std::thread::spawn(msg_excited);

    let hello = hello.join().expect("to return hello");
    let thread = thread.join().expect("to return threads");
    let excited = excited.join().expect("to return ! mark");

    println!("{} {} {}", hello, thread, excited);
}
