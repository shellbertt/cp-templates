#![allow(arithmetic_overflow)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
use std::io::{self, BufRead, Write};
use std::cmp::{self, min, max};
use std::ops::{self, Add, Sub, Neg, Mul, Div, Rem, Index};
use std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap, LinkedList};
use std::cell::RefCell;
use std::mem::swap;

#[allow(dead_code)]
pub type Pair = (i32, i32);

fn parse_input() -> io::Result<()>
{
    let stdin = io::stdin();

    // Manual iterator over lines. Most common
    let mut lines = stdin.lock().lines().map(|s| s.expect("Failed to read line"));
    // // General iterator over lines
    // let mut lines = stdin.lock().lines();
    // while let Some(Ok(line)) = lines.next() {
    // }
    // // To parse comma separated integers into a vec like: 1, 2, 3
    // let mut ints = Vec::with_capacity(1 << 20);
    // lines.for_each(|s|
    //     s.split(", ")
    //     .map(|n| n.parse::<i32>().expect("Failed to parse int"))
    //     .for_each(|n| ints.push(n))
    // );
    // // Buffering reads
    // let mut input = String::new();  // Remember to clear. read_line() appends to current string
    // stdin.lock().read_line(&mut input).expect("Failed to read input line");
    // // For splitting across generalized strings
    // stdin.lock().lines().next().unwrap().unwrap();
    // let items = stdin.lock().split(b',');  // one,two,three
    // // Very low-level api for C speeds
    // let mut stream = stdin.lock();
    // let mut buf = Vec::with_capacity(1 << 20);
    // let mut void = io::sink();  // Alternative option to discard output. Use io::Empty in v1.62
    // let length = stream.read_until(b'\n', &mut buf).unwrap();  // Includes last '\n' byte
}

fn print_output() -> io::Result<()>
{
    let mut writebuf = io::BufWriter::new(io::stdout());

    // Use write!() for formatting, .write_all() otherwise. Unwrap both results
    write!(&mut writebuf, "{}: 11", "SegFault")?;
    writebuf.write_all(b"\n")?;

    writebuf.flush()
}

fn main()
{
    let input = parse_input();

    print_output()
        .expect("Failed to print output to stdout");
}
