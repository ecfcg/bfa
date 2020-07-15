use md5::{Digest, Md5};

const ASCII: &'static str = r##" abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()~+=_-[]{}:`\|'"<>?,./;"##;

struct RawStr {
    all_chars: Vec<u8>,
    max_point: usize,
    value: Vec<usize>,
    point: usize,
}

impl RawStr {
    fn new() -> Self {
        let all_chars = ASCII
            .as_bytes()
            .into_iter()
            .map(|&u| u)
            .collect::<Vec<u8>>();
        let max_point = all_chars.len();

        RawStr {
            all_chars: all_chars,
            max_point: max_point,
            value: vec![0],
            point: 0,
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
        self.value = Vec::with_capacity(self.value.len() + 1);
        self.point += 1;

        let mut devided = self.point;
        loop {
            self.value.push(devided % self.max_point);
            devided /= self.max_point;
            if devided == 0 {
                break;
            }
        }
    }
}

pub struct Md5Decrypter {
    lut: Vec<String>,
}

impl Md5Decrypter {
    pub fn new() -> Self {
        Md5Decrypter {
            lut: (0u8..=255).map(|n| format!("{:02X}", n)).collect(),
        }
    }

    pub fn decrypt(self: &Self, hash: String, max_len: usize) {
        let mut raw = RawStr::new();
        loop {
            let raw_str = raw.next();
            if raw_str.len() > max_len {
                break;
            }

            if self.compare(&raw_str, &hash) {
                println!("{}", raw_str);
                return;
            }
        }
    }

    fn compare(self: &Self, raw_str: &String, hash: &str) -> bool {
        let mut hasher = Md5::new();
        hasher.update(raw_str.as_bytes());
        let hashed = hasher
            .finalize()
            .as_slice()
            .iter()
            .map(|&u| self.lut.get(u as usize).unwrap().to_owned())
            .collect::<String>();

        *hash == hashed
    }
}
