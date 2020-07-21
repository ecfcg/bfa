use md5::{Digest, Md5};
use std::assert;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

mod code_points;
use code_points::CodePoints;

pub use code_points::parse_start_code;

pub struct Clacker {
    max_len: usize,
    thread_num: usize,
    start_code: Vec<usize>,
    hex: Arc<Vec<u8>>,
    finished: Arc<AtomicBool>,
    canceled: Arc<AtomicBool>,
    saved_cp: Arc<Mutex<Vec<Vec<usize>>>>,
}

impl Clacker {
    pub fn new(hex_str: String, max_len: usize, thread_num: usize, start_code: Vec<usize>) -> Self {
        Clacker {
            max_len: max_len,
            thread_num: thread_num,
            start_code: start_code,
            hex: Arc::new(to_hex(hex_str)),
            finished: Arc::new(AtomicBool::new(false)),
            canceled: Arc::new(AtomicBool::new(false)),
            saved_cp: Arc::new(Mutex::new(Vec::with_capacity(thread_num))),
        }
    }

    pub fn clack(self: &Self) {
        let mut threads = Vec::with_capacity(self.thread_num);
        for start in 0..self.thread_num {
            threads.push(self.decode(start));
        }
        self.wait_cancel();
        for handle in threads {
            handle.join().unwrap();
        }
        let fini = self.finished.load(Ordering::Relaxed);
        let canc = self.canceled.load(Ordering::Relaxed);
        if !fini {
            if canc {
                println!("Canceled.");
                self.to_saved_str();
            } else {
                println!("Not found.");
                let next = (0..self.max_len)
                    .into_iter()
                    .map(|_| String::from("0"))
                    .collect::<Vec<String>>()
                    .join("-");
                println!("{}", next);
            }
        }
    }

    fn decode(self: &Self, start: usize) -> JoinHandle<()> {
        let thread_num = self.thread_num;
        let max_len = self.max_len;
        let start_code = self.start_code.clone();

        let arc_hex_val = self.hex.clone();
        let arc_finished = self.finished.clone();
        let arc_canceled = self.canceled.clone();
        let arc_saved = self.saved_cp.clone();

        thread::spawn(move || {
            let mut cps = CodePoints::new(thread_num, start, start_code);
            let mut checked = 0;

            loop {
                let cp = cps.next();
                let hashed_cp = to_hash(&cp);

                if cp.len() > max_len {
                    return;
                }

                if *arc_hex_val == hashed_cp {
                    arc_finished.store(true, Ordering::Relaxed);
                    println!("{}", String::from_utf8(cp).unwrap());
                    return;
                }

                checked += 1;

                if checked == 10000 {
                    if arc_finished.load(Ordering::Relaxed) {
                        return;
                    } else if arc_canceled.load(Ordering::Relaxed) {
                        let mut saved = arc_saved.lock().unwrap();
                        (*saved).push(cps.value());
                        return;
                    }
                    checked = 0;
                }
            }
        })
    }

    fn wait_cancel(self: &Self) {
        let arc_canceled = self.canceled.clone();
        thread::spawn(move || loop {
            let mut s = String::default();

            match std::io::stdin().read_line(&mut s) {
                Ok(_) => (),
                Err(_) => continue,
            }

            let cancel = match s.trim() {
                "q" | "Q" => true,
                _ => continue,
            };

            if cancel {
                arc_canceled.store(true, Ordering::Relaxed);
                return;
            }
        });
    }

    fn to_saved_str(self: &Self) {
        let saved = self.saved_cp.lock().unwrap();

        let mut saved_str = (*saved)
            .iter()
            .map(|v| {
                v.iter()
                    .skip(1)
                    .map(|i| format!("{:02}", i))
                    .collect::<Vec<String>>()
                    .join("-")
            })
            .collect::<Vec<String>>();

        saved_str.sort();
        println!("{}", saved_str[0]);
    }
}

fn to_hex(hash: String) -> Vec<u8> {
    assert!(hash.len() % 2 == 0);
    let mut v = Vec::with_capacity(hash.len() / 2);
    let c = hash.chars().collect::<Vec<char>>();
    let mut n = 0;
    for i in 0..c.len() {
        let x = match c[i] {
            '0'..='9' => c[i] as u8 - '0' as u8,
            'a'..='f' => c[i] as u8 - 'a' as u8 + 10,
            'A'..='F' => c[i] as u8 - 'A' as u8 + 10,
            _ => panic!(format!("Not hex character:{}", c[i])),
        };

        if i % 2 == 0 {
            n = x << 4;
        } else {
            n += x;
            v.push(n);
            n = 0;
        }
    }

    v
}

fn to_hash(code_points: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Md5::new();
    hasher.update(code_points.as_slice());
    hasher.finalize().as_slice().iter().map(|&u| u).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(to_hex(String::from("01")), vec![1]);
        assert_eq!(
            to_hex(String::from("0123456789ABCDEFabcdef")),
            vec![0x1, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xab, 0xcd, 0xef]
        );
    }
}
