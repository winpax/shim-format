use core::fmt::Display;

use alloc::string::String;

use crate::Shim;

impl Shim {
    pub(crate) fn inner_to_string(&self) -> String {
        let mut output = String::new();

        output += self.path();

        if !self.args().is_empty() {
            output += "\r\n";
            output += &self.args().join(" ");
        }

        output
    }
}

impl Display for Shim {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let shim_string = self.inner_to_string();

        f.write_str(&shim_string)
    }
}
