use arbitrary_int::u3;
use bitmacro2::{bitfield, bitfield_enum};

fn main() {
    let mut flags = PageTableEntryFlags::empty()
        .with_perms(Permissions::empty().with_read(true).with_execute(true))
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
        pub perms: Permissions,

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

bitfield! {
    pub struct Permissions: u3 {
        pub read: bool,
        pub write: bool,
        pub execute: bool,
    }
}

bitfield_enum! {
    #[derive(Default)]
    pub enum PermissionsV2: u3 {
        #[default]
        Unknown = 0,

        Read = 0b01,
        Write = 0b10,
        Execute = 0b100,

        ReadWrite = 0b11,
        ReadExecute = 0b101,
        ReadWriteExecute = 0b111,
    }
}
