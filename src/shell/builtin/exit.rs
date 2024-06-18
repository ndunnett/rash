use std::process::exit;

use super::Builtin;
use crate::shell::{IOContext, Shell};

pub struct Exit {
    code: i32,
}

impl Builtin for Exit {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        let code = args
            .first()
            .map(|arg| arg.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);

        Ok(Box::new(Self { code }))
    }

    fn run(&self, _sh: &mut Shell, _io: &mut IOContext) -> i32 {
        exit(self.code)
    }
}
