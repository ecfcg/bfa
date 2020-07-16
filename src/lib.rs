use md5::{Digest, Md5};
use rayon::prelude::*;
use std::sync::Mutex;

const ASCII: &'static str = r##" abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()~+=_-[]{}:`\|'"<>?,./;"##;

struct RawStr {
    all_chars: Vec<u8>,
    max_point: usize,
    value: Vec<usize>,
    step: usize,
}

impl RawStr {
    fn new(step: usize, start: usize) -> Self {
        let all_chars = ASCII
            .as_bytes()
            .into_iter()
            .map(|&u| u)
            .collect::<Vec<u8>>();
        let max_point = all_chars.len();

        RawStr {
            all_chars: all_chars,
            max_point: max_point,
            value: vec![start],
            step: step,
        }
    }

    fn next(self: &mut Self) -> String {
        let v = self
            .value
            .iter()
            .map(|&p| self.all_chars[p])
            .collect::<Vec<u8>>();

        self.increment();
        String::from_utf8(v).unwrap()
    }

    fn increment(self: &mut Self) {
        let mut new_value = Vec::with_capacity(self.value.len() + 1);
        let point;

        point = self.value[0] + self.step;

        if point >= self.max_point {
            new_value.push(point % self.max_point);
            new_value.append(&mut self.carry());
        } else {
            new_value.push(point);
            new_value.append(&mut self.value.iter().skip(1).map(|&i| i).collect());
        }

        self.value = new_value;
    }

    fn carry(self: &Self) -> Vec<usize> {
        let mut carried = Vec::with_capacity(self.value.len());
        let mut it = self.value.iter().skip(1);
        let mut point;
        loop {
            point = match it.next() {
                Some(i) => i + 1,
                None => 0,
            };

            if point == self.max_point {
                carried.push(0);
            } else {
                carried.push(point);
                carried.append(&mut it.map(|&i| i).collect());
                break;
            }
        }

        carried
    }
}

pub struct Md5Decrypter {}

impl Md5Decrypter {
    pub fn new() -> Self {
        Md5Decrypter {}
    }

    pub fn decrypt(self: &Self, hash: String, max_len: usize, thread_num: usize) {
        let lut = (0u8..=255).map(|n| format!("{:02X}", n)).collect();
        let xs: Vec<usize> = (0..thread_num).collect();
        let m = Mutex::new(false);

        xs.par_iter().for_each(|&start| {
            let mut raw = RawStr::new(thread_num, start);
            let mut i = 0;

            loop {
                let raw_str = raw.next();

                if raw_str.len() > max_len {
                    return;
                }

                if self.compare(&raw_str, &hash, &lut) {
                    println!("{}", raw_str);
                    let mut flag = m.lock().unwrap();
                    *flag = true;
                    return;
                }

                i += 1;

                if i == 10000 {
                    i = 0;
                    let flag = m.lock().unwrap();

                    if *flag {
                        return;
                    }
                }
            }
        });
    }

    fn compare(self: &Self, raw_str: &String, hash: &str, lut: &Vec<String>) -> bool {
        let mut hasher = Md5::new();
        hasher.update(raw_str.as_bytes());
        let hashed = hasher
            .finalize()
            .as_slice()
            .iter()
            .map(|&u| lut.get(u as usize).unwrap().to_owned())
            .collect::<String>();
        *hash == hashed
    }
}
