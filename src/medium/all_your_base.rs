#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 { return Err(Error::InvalidInputBase) }
    if to_base < 2 { return Err(Error::InvalidOutputBase) }
    if number.is_empty() { return Ok(vec![0]); }

    let mut decimal: u32 = 0;
    for (i, &n) in number.iter().rev().enumerate() {
        if n >= from_base { return Err(Error::InvalidDigit(n)) }
        decimal += from_base.pow(i as u32) * n;
    }

    let mut res = Vec::new();
    res.push(decimal % to_base);
    while decimal / to_base > 0 {
        decimal /= to_base;
        res.push(decimal % to_base);
    }
    res.reverse();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(convert(&[], 2, 10), Ok(vec![0]));
        assert_eq!(convert(&[1], 2, 10), Ok(vec![1]));
        assert_eq!(convert(&[1, 0, 1], 2, 10), Ok(vec![5]));
        assert_eq!(convert(&[5], 10, 2), Ok(vec![1, 0, 1]));
        assert_eq!(convert(&[1, 0, 1, 0, 1, 0], 2, 10), Ok(vec![4, 2]));
        assert_eq!(convert(&[1, 1, 2, 0], 3, 16), Ok(vec![2, 10]));
        assert_eq!(convert(&[3, 46, 60], 97, 73), Ok(vec![6, 10, 45]));
    }

    #[test]
    fn err() {
        assert_eq!(convert(&[1, 2, 1, 0, 1, 0], 2, 10), Err(Error::InvalidDigit(2)));
        assert_eq!(convert(&[0], 1, 10), Err(Error::InvalidInputBase));
        assert_eq!(convert(&[1], 2, 1), Err(Error::InvalidOutputBase));
    }
}
