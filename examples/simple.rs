use bitmacro2::bitfield;

fn main() {
    //
}

bitfield! {
    pub struct RiscvPageTableEntryFlags: u8 {
        pub valid: bool,
        pub read: bool,
        pub write: bool,
        pub execute: bool,
        pub user: bool,
        pub global: bool,
        pub accessed: bool,
        pub dirty: bool,
    }
}
