pub(crate) struct Interrupts {
    pub(crate) enabled: bool,
    pub(crate) hblank: bool,
    pub(crate) lcd: bool,
    pub(crate) oam: bool,
    pub(crate) vblank: bool,
}
