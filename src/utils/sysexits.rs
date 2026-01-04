#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum ExitCode {
    Usage = 64,     // you ran it wrong
    DataErr = 65,   // input is cursed
    NoInput = 66,   // file does not exist in this reality
    CantCreat = 73, // OS refused to make the file
    IoErr = 74,     // disk said nope
    Software = 70,  // this one is on us
}

pub fn exit(code: ExitCode) -> ! {
    std::process::exit(code as i32)
}
