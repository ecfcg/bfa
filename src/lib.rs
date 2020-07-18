use md5::{Digest, Md5};
use std::assert;
use std::sync::{Arc, Mutex};
use std::thread;

mod code_points;
use code_points::CodePoints;

pub fn clack(hex_str: String, max_len: usize, thread_num: usize) {
    let hex = Arc::new(to_hex(hex_str));
    let finished = Arc::new(Mutex::new(false));
    let mut threads = Vec::with_capacity(thread_num);

    for start in 0..thread_num {
        let arc_hex_val = hex.clone();
        let arc_finished = finished.clone();

        threads.push(thread::spawn(move || {
            let mut cps = CodePoints::new(thread_num, start);
            let mut checked = 0;

            loop {
                let cp = cps.next();
                let hashed_cp = to_hash(&cp);

                if cp.len() > max_len {
                    return;
                }

                if *arc_hex_val == hashed_cp {
                    let mut flag = arc_finished.lock().unwrap();
                    *flag = true;
                    println!("{}", String::from_utf8(cp).unwrap());
                    return;
                }

                checked += 1;

                if checked == 10000 {
                    let flag = arc_finished.lock().unwrap();
                    if *flag {
                        return;
                    }
                    checked = 0;
                }
            }
        }));
    }

    for handle in threads {
        handle.join().unwrap();
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
