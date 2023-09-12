use crate::parser::parser::LispVal;

#[derive(Debug, Clone)]
struct EnvFrame(Vec<(String, LispVal)>);

impl EnvFrame {
    pub(crate) fn new() -> EnvFrame {
        EnvFrame(Vec::new())
    }

    pub(crate) fn lookup(&self, key: &str) -> Option<&LispVal> {
        self.0
            .iter()
            .rev()
            .find(|entry| entry.0.eq(key))
            .map(|pair| &pair.1)
    }
}

#[derive(Debug)]
pub struct Environment(Vec<EnvFrame>);

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment(Vec::new());
        env.new_frame();
        env
    }

    pub fn lookup(&self, key: &str) -> Option<&LispVal> {
        self.0
            .iter()
            .rev()
            .find_map(|frame: &EnvFrame| frame.lookup(key))
    }

    pub fn new_frame(&mut self) -> &mut Self {
        self.0.push(EnvFrame::new());
        self
    }

    pub fn pop_frame(&mut self) -> &mut Self {
        self.0.pop();
        self
    }

    pub fn new_binding(&mut self, key: String, value: LispVal) {
        self.0
            .last_mut()
            .expect("no frame to bind to")
            .0
            .push((key, value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let mut env = Environment::new();
        env.new_binding("a".to_string(), LispVal::Integer(1));
        env.new_frame()
            .new_binding("a".to_string(), LispVal::Integer(2));
        assert_eq!(
            env.lookup("a"),
            Some(&LispVal::Integer(2)),
            "should find inner binding"
        );
        env.pop_frame();
        assert_eq!(
            env.lookup("a"),
            Some(&LispVal::Integer(1)),
            "should find outer binding"
        );
    }
}
