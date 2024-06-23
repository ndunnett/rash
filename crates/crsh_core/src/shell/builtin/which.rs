use super::{Builtin, ImplementedBuiltin};
use crate::shell::{IOContext, Shell};

pub struct Which {
    keyword: String,
}

impl ImplementedBuiltin for Which {
    fn build(args: &[&str]) -> Result<impl ImplementedBuiltin, String> {
        let keyword = args.first().map(|&s| s.to_string()).unwrap_or_default();
        Ok(Self { keyword })
    }

    fn run(&self, sh: &mut Shell, io: &mut IOContext) -> i32 {
        if Builtin::get(&self.keyword).is_some() {
            io.println(format!("{}: shell builtin", self.keyword));
        } else if let Some(path) = sh.find_on_path(&self.keyword) {
            io.println(path.display().to_string());
        } else {
            io.println(format!("{} not found", self.keyword));
        }

        0
    }
}