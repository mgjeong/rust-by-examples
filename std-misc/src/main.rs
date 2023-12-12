// ---------------------------------------------------------------------
// section 01 - threads
/*
// Rust provides a mechanism for spawning native OS threads via the `spawn` function,
// the argument of this function is a moving closure.

use std::thread;

const NTHREAD: u32 = 10;

fn main() {
    let mut children = vec![];

    for i in 0..NTHREAD {
        children.push(thread::spawn(move || {
            println!("this is thread number: {}", i);
        }));
    }

    for child in children {
        // wait for the thread to finish. return a result.
        let _ = child.join();
    }
}
*/

// ---------------------------------------------------------------------
// section 02 - threads: testcase, map-reduce
/*
// Rust makes it very easy to parallelise data processing, without many of the headaches
// traditionally associated with such an attempt.
//
// The standard library provides great threading primitives out of the box.
// These, combined with Rust's concept of Ownership and aliasing rules, automatically prevent data races.
//
// The aliasing rules (one writable reference XOR many readable references) automatically prevent you
// from manipulating state that is visible to other threads. (Where synchronisation is needed,
// there are synchronisation primitives like `Mutexes` or `Channels`.)
//
// In this example, we will calculate the sum of all digits in a block of numbers.
// We will do this by parcelling out chunks of the block into different threads.
// Each thread will sum its tiny block of digits, and subsequently we will sum the intermediate sums
// produced by each thread.
//
// Note that, although we're passing references across thread boundaries, Rust understands that
// we're only passing read-only references, and that thus no unsafety or data races can occur.
// Also because the references we're passing have `'static` lifetimes, Rust understands that
// our data won't be destroyed while these threads are still running. (When you need to share
// non-`static` data between threads, you can use a smart pointer like `Arc` to keep the data alive
// and avoid non-`static` lifetimes.)

use std::thread;

fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    let mut children = vec![];

    // map phase
    // devide our data into segments, and apply initial processing

    // split out data into segments for individual calculation
    // each chunk will be a reference( &str ) intothe actual data
    let chunked_data = data.split_whitespace();
    println!("{:?}", chunked_data);

    // iterate over the data segments
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two varaibales, "i" and "data_segment" with a
    // "destructuring assignement"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // processes each data segment in a separate thread
        // spawn() returns a handle to the new thread
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no argument ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.

        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                // iterate over the characters of our segment
                .chars()
                // convert text-characters to their number value
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // and sum the resulting iterator of numbers
                .sum();
            println!("processed segment {}, result={}", i, result);

            result
        }));
    }

    // reduce phase
    // collect our intermediate results, and combine them into a final result

    // combine each thread's intermediate results into a single final sum
    // we use the "turbofish" ::<> to provide sum() with a type hint.

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("final sum result {}", final_result);
}
*/

// ---------------------------------------------------------------------
// section 03 - channels
/*
// Rust provides asynchronous `channel`s for communication between threads.
// Channels allow a unidirectional flow of information between two end-points:
// the `Sender` and the `Receiver`.
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred.
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // the sender endpoint can be copied.
        let thread_tx = tx.clone();

        // each thread will send its id via the channel
        let child = thread::spawn(move || {
            // the thread takes ownership over `thread_tx`
            // each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished.", id);
        });

        children.push(child);
    }

    // here, all the messages are collected.
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // the `recv` method picks a messgae frrom the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    for id in ids {
        println!("id: {}", id.unwrap());
    }
}
*/

// ---------------------------------------------------------------------
// section 04 - path
/*
// The `Path` struct represents file paths in the underlying filesystem.
// There are two flavors of `Path`: `posix::Path`, for UNIX-like systems, and `windows::Path`, for Windows.
// The prelude exports the appropriate platform-specific `Path` variant.
//
// A Path can be created from an `OsStr`, and provides several methods to get information
// from the file/directory the path points to.
//
// A `Path` is immutable. The owned version of `Path` is `PathBuf`. The relation between `Path` and `PathBuf`
// is similar to that of `str` and `String`: a `PathBuf` can be mutated in-place, and can be dereferenced to a `Path`.
//
// Note that a `Path` is not internally represented as an UTF-8 string, but instead is stored as an `OsString`.
// Therefore, converting a `Path` to a `&str` is not free and may fail (an `Option` is returned).
// However, a `Path` can be freely converted to an `OsString` or `&OsStr` using `into_os_string` and `as_os_str`, respectively.

use std::path::Path;

fn main() {
    // create a `path` from an `&'static str`
    let path = Path::new(".");

    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific separateor and returns a `PathBuf`
    let mut new_path = path.join("a").join("b");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates the file name of the `PathBuf`
    new_path.set_file_name("package.tgz");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
*/

// ---------------------------------------------------------------------
// section 05 - file io: open
/*
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contanins: \n{}", display, s),
    }
}
*/

// ---------------------------------------------------------------------
// section 06- file io: create
/*
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
*/

// ---------------------------------------------------------------------
// section 07 - file io: read lines
/*
// use std::fs::read_to_string;

// fn read_lines(filename: &str) -> Vec<String> {
//     // beginner's approach
//     // let mut result = Vec::new();
//     // for line in read_to_string(filename).unwrap().lines() {
//     //     result.push(line.to_string());
//     // }

//     // result

//     // more concise expression
//     read_to_string(filename)
//         .unwrap()
//         .lines()
//         .map(String::from)
//         .collect()
// }

// more effiecient approach
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./hosts.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    } else {
        println!("fail to open the hosts.txt");
    }
}
*/

// ---------------------------------------------------------------------
// section 08 - child processes
/*
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }
}
*/

// ---------------------------------------------------------------------
// section 09 - child processes: pipes
/*
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

fn main() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => println!("wc responded with: {}", s),
    }
}
*/

// ---------------------------------------------------------------------
// section 10 - child processes: wait
/*
 */

use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}

// ---------------------------------------------------------------------
// section 11 - file operations
/*
 */

// ---------------------------------------------------------------------
// section 12 - program arguments
/*
 */

// ---------------------------------------------------------------------
// section 13 - program arguments: argument parsing
/*
 */

// ---------------------------------------------------------------------
// section 14 - foreign function interface
/*
 */
