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
        Environment(Vec::new())
    }

    pub fn lookup(&self, key: &str) -> Option<&LispVal> {
        self.0
            .iter()
            .rev()
            .find_map(|frame: &EnvFrame| frame.lookup(key))
    }

    pub fn new_frame(&mut self) {
        self.0.push(EnvFrame::new())
    }
}
