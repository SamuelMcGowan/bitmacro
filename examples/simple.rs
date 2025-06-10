use bitmacro2::bitfield;

fn main() {
    let flags = PageTableEntryFlags::empty()
        .with_valid(true)
        .with_read(true);
}

bitfield! {
    pub struct PageTableEntryFlags: u8 {
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
