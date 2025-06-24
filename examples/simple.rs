use arbitrary_int::u3;
use bitmacro::{bitfield, bitfield_enum};

fn main() {
    let mut flags = PageTableEntryFlags::empty()
        .with_permissions(Permissions::ReadExecute)
        .with_valid(true);

    flags.set_valid(true);

    dbg!(flags);
}

bitfield! {
    /// Page table flags.
    #[derive(PartialEq, Eq)]
    pub struct PageTableEntryFlags: u8 {
        /// Whether the page is valid.
        pub valid: bool,

        /// Page permissions.
        pub permissions: Permissions,

        /// Whether this is a user page.
        pub user: bool,

        /// Whether this is a global page.
        pub global: bool,

        /// Whether this page has been accessed.
        pub accessed: bool,

        /// Whether this page has been written to.
        pub dirty: bool,
    }
}

bitfield_enum! {
    pub enum Permissions: u3 {
        Read = 0b01,
        Write = 0b10,
        Execute = 0b100,

        ReadWrite = 0b11,
        ReadExecute = 0b101,
        ReadWriteExecute = 0b111,

        Reserved0 = 0,
        Reserved1 = 0b110,
    }
}
