use arbitrary_int::u3;
use bitmacro2::bitfield;

fn main() {
    let flags = PageTableEntryFlags::empty()
        .with_perms(Permissions::empty().with_read(true).with_execute(true))
        .with_valid(true);

    dbg!(flags);
}

bitfield! {
    pub struct PageTableEntryFlags: u8 {
        pub valid: bool,
        pub perms: Permissions,
        pub user: bool,
        pub global: bool,
        pub accessed: bool,
        pub dirty: bool,
    }
}

bitfield! {
    pub struct Permissions: u3 {
        pub read: bool,
        pub write: bool,
        pub execute: bool,
    }
}
