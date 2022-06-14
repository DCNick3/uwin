struct ErrorRecord {
    #[allow(unused)]
    code: u32,
    #[allow(unused)]
    slug: &'static str,
    desc: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

impl Win32Error {
    pub fn get_description(self) -> &'static str {
        let code = self as u32;
        let record = ERROR_CODES.get(&code).unwrap();
        record.desc
    }
    pub fn get_code(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod test {
    use super::Win32Error;

    #[test]
    fn smoke_test() {
        let error = Win32Error::ERROR_ALREADY_EXISTS;

        assert_eq!(
            error.get_description(),
            "Cannot create a file when that file already exists."
        );
        assert_eq!(error.get_code(), 183);
    }
}
