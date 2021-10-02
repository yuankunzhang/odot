use std::str::FromStr;

#[derive(Debug)]
pub enum Tag {
    Project(String),
    Context(String),
    KV(String, String),
}

impl FromStr for Tag {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let marker = s.chars().nth(0).ok_or("invalid tag string")?;
        match marker {
            '+' | '@' => {
                let key = &s[1..];

                if key.is_empty() {
                    return Err("invalid tag string");
                }

                if marker == '+' {
                    Ok(Tag::Project(key.to_owned()))
                } else {
                    Ok(Tag::Context(key.to_owned()))
                }
            }
            _ => {
                let mut split = s.split(":");
                let key = split.next().unwrap();
                let val = split.collect::<Vec<&str>>().join(":");
                Ok(Tag::KV(key.to_owned(), val.to_owned()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag_from_str() {
        assert!(matches!("".parse::<Tag>(), Err("invalid tag string")));
        assert!(matches!("+".parse::<Tag>(), Err("invalid tag string")));
        assert!(matches!("@".parse::<Tag>(), Err("invalid tag string")));

        match "+eagle".parse::<Tag>() {
            Ok(Tag::Project(s)) => assert_eq!(s, "eagle"),
            _ => assert!(false)
        }

        match "@red".parse::<Tag>() {
            Ok(Tag::Context(s)) => assert_eq!(s, "red"),
            _ => assert!(false)
        }

        match "age:17".parse::<Tag>() {
            Ok(Tag::KV(k, v)) => {
                assert_eq!(k, "age");
                assert_eq!(v, "17");
            }
            _ => assert!(false)
        }

        match "age:".parse::<Tag>() {
            Ok(Tag::KV(k, v)) => {
                assert_eq!(k, "age");
                assert!(v.is_empty());
            }
            _ => assert!(false)
        }

        match "age".parse::<Tag>() {
            Ok(Tag::KV(k, v)) => {
                assert_eq!(k, "age");
                assert!(v.is_empty());
            }
            _ => assert!(false)
        }
    }
}
