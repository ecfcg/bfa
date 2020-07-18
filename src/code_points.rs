use std::assert;

const ASCII: &'static str = r##" abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()~+=_-[]{}:`\|'"<>?,./;"##;

pub(crate) struct CodePoints {
    all_chars: Vec<u8>,
    max_point: usize,
    value: Vec<usize>,
    step: usize,
}

impl CodePoints {
    pub(crate) fn new(step: usize, start: usize) -> Self {
        assert!(start < step);
        assert!(0 < step);

        let all_chars = ASCII.as_bytes().iter().map(|&u| u).collect::<Vec<u8>>();
        let max_point = all_chars.len();

        assert!(step < max_point);

        CodePoints {
            all_chars: all_chars,
            max_point: max_point,
            value: vec![start],
            step: step,
        }
    }

    pub(crate) fn next(self: &mut Self) -> Vec<u8> {
        let v = self
            .value
            .iter()
            .map(|&p| self.all_chars[p])
            .collect::<Vec<u8>>();

        self.increment();
        v
    }

    fn increment(self: &mut Self) {
        let mut point = self.value[0] + self.step;

        if self.max_point <= point {
            // 桁上がりする場合は計算を続行
            self.value[0] = point % self.max_point;

            for i in 1..self.value.len() {
                point = self.value[i] + 1;

                if point == self.max_point {
                    self.value[i] = 0;
                } else {
                    // 桁上がりしない場合は対象桁を更新して終了
                    self.value[i] = point;
                    return;
                }
            }

            // 桁が増える場合
            self.value.push(0);
        } else {
            // 桁上がりしない場合は最小桁のみ更新
            self.value[0] = point;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let points = CodePoints::new(5, 3);
        assert_eq!(points.max_point, 95);
        assert_eq!(points.value, vec![3]);
        assert_eq!(points.step, 5);
    }

    #[test]
    fn test_next() {
        let mut points = CodePoints::new(5, 3);
        assert_eq!(points.next(), vec!['c' as u8]);
        assert_eq!(points.next(), vec!['h' as u8]);
    }

    #[test]
    fn test_increment() {
        let mut points = CodePoints::new(5, 3);
        assert_eq!(points.value, vec![3]);

        points.increment();
        assert_eq!(points.value, vec![8]);

        points.value = vec![93];
        points.increment();
        assert_eq!(points.value, vec![3, 0]);

        points.value = vec![94, 94];
        points.increment();
        assert_eq!(points.value, vec![4, 0, 0]);
    }
}
