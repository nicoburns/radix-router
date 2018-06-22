#[derive(Debug)]
pub struct Param {
    key: String,
    value: String,
}

#[derive(Debug)]
pub struct Params(Vec<Param>);

impl Params {
    pub fn by_name(&self, name: &str) -> Option<String> {
        match self.0.iter().find(|param| param.key == name) {
            Some(param) => Some(param.value.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn params() {
        use router::{Param, Params};

        let params = Params(vec![
            Param {
                key: "fuck".to_owned(),
                value: "you".to_owned(),
            },
            Param {
                key: "lalala".to_string(),
                value: "papapa".to_string(),
            },
        ]);

        assert_eq!(Some(String::from("you")), params.by_name("fuck"));
        assert_eq!(Some(String::from("papapa")), params.by_name("lalala"));
    }
}