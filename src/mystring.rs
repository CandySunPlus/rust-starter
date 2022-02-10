use std::{fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

pub struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new<T: AsRef<str>>(v: T) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
pub enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl MyString {
    pub fn push_str(&mut self, s: &str) {
        match *self {
            MyString::Inline(ref mut v) => {
                let len = v.len();
                let new_bytes = s.as_bytes();
                let new_len = len + new_bytes.len();
                match new_len > MINI_STRING_MAX_LEN {
                    true => *self = MyString::Standard(format!("{}{}", self, s)),
                    _ => {
                        v.data[len..new_len].copy_from_slice(new_bytes);
                        v.len = new_len as u8;
                    }
                }
            }
            MyString::Standard(ref mut v) => {
                v.push_str(s);
            }
        }
    }
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl<T> From<T> for MyString
where
    T: AsRef<str> + Into<String>,
{
    fn from(s: T) -> Self {
        match s.as_ref().len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.into()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl PartialEq for MyString {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
}

impl PartialEq<MyString> for str {
    fn eq(&self, other: &MyString) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
}

impl<'a> PartialEq<MyString> for &'a str {
    fn eq(&self, other: &MyString) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
}

impl PartialEq<str> for MyString {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
}

impl<'a> PartialEq<&'a str> for MyString {
    fn eq(&self, other: &&'a str) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_string() {
        let len1 = std::mem::size_of::<MyString>();
        let len2 = std::mem::size_of::<MiniString>();
        let len3 = std::mem::size_of::<String>();

        assert_eq!(32, len1);
        assert_eq!(31, len2);
        assert_eq!(24, len3);

        let s1: MyString = "你好".into();
        let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();
        let mut s3: MyString = String::from("small string").into();
        assert_eq!("你好", s1.deref());
        assert_eq!("这是一个超过了三十个字节的很长很长的字符串", s2.deref());
        assert_eq!("small string", s3);
        s3.push_str("hello");
        assert_eq!(s3, "small stringhello");
        s3.push_str(&s2);
        assert_eq!(
            "small stringhello这是一个超过了三十个字节的很长很长的字符串",
            s3
        );

        assert_eq!(6, s1.len());
        assert_eq!(2, s1.chars().count());
        assert_eq!(63, s2.len());
        assert_eq!(21, s2.chars().count());

        assert!(s1.ends_with('好'));
        assert!(s2.starts_with('这'));
        assert!(s3.ends_with("字符串"));
    }
}
