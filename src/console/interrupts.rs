pub(crate) struct Interrupts {
    /*
    FFFF — IE: Interrupt enable

    Bit 0: VBlank   Interrupt Enable  (INT $40)  (1=Enable)
    Bit 1: LCD STAT Interrupt Enable  (INT $48)  (1=Enable)
    Bit 2: Timer    Interrupt Enable  (INT $50)  (1=Enable)
    Bit 3: Serial   Interrupt Enable  (INT $58)  (1=Enable)
    Bit 4: Joypad   Interrupt Enable  (INT $60)  (1=Enable)
    */
    pub(crate) vblank_enable: bool,
    pub(crate) lcd_stat_enable: bool,
    pub(crate) timer_enable: bool,
    pub(crate) serial_enable: bool,
    pub(crate) joypad_enable: bool,

    /*
    FF0F — IF: Interrupt flag

    Bit 0: VBlank   Interrupt Request (INT $40)  (1=Request)
    Bit 1: LCD STAT Interrupt Request (INT $48)  (1=Request)
    Bit 2: Timer    Interrupt Request (INT $50)  (1=Request)
    Bit 3: Serial   Interrupt Request (INT $58)  (1=Request)
    Bit 4: Joypad   Interrupt Request (INT $60)  (1=Request)
    */
    pub(crate) vblank_request: bool,
    pub(crate) lcd_stat_request: bool,
    pub(crate) timer_request: bool,
    pub(crate) serial_request: bool,
    pub(crate) joypad_request: bool,
}

impl Interrupts {
    pub(crate) fn new() -> Interrupts {
        Interrupts {
            vblank_enable: false,
            lcd_stat_enable: false,
            timer_enable: false,
            serial_enable: false,
            joypad_enable: false,
            vblank_request: false,
            lcd_stat_request: false,
            timer_request: false,
            serial_request: false,
            joypad_request: false,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.vblank_enable = false;
        self.lcd_stat_enable = false;
        self.timer_enable = false;
        self.serial_enable = false;
        self.joypad_enable = false;
        self.vblank_request = false;
        self.lcd_stat_request = false;
        self.timer_request = false;
        self.serial_request = false;
        self.joypad_request = false;
    }
}

