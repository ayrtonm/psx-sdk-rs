#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    use crate::bios::printf;
    use crate::framebuffer::Framebuffer;
    use crate::gpu::*;
    use crate::hal::MutRegister;
    use crate::hal::{I_MASK, I_STAT};
    use crate::interrupt::IRQ;
    use crate::printer::Printer;

    static mut pr: Printer = Printer::new((0, 0), (0, 0), (320, 240), None);
    static mut fb: Framebuffer = Framebuffer::new((0, 0), (0, 240), (320, 240), None);
    unsafe {
        pr.load_font();

        reset_graphics((320, 240), VideoMode::NTSC, Depth::High, false);
        I_MASK::load_mut()
            .disable_all()
            .enable_irq(IRQ::Vblank)
            .store();

        let start = b"Running all tests...\n\0";
        printf(&start[0], 0);
        for t in tests {
            t();
        }
        let msg = b"\nPassed all tests!\n\0";
        printf(&msg[0], 0);
        loop {
            pr.reset();
            pr.print(msg, []);
            I_STAT::load_mut()
                .ack(IRQ::Vblank)
                .store()
                .wait(IRQ::Vblank);
            fb.swap();
        }
    }
}

// This is mainly an example of how to write tests
// Note the test attribute for custom test frameworks is different
#[cfg(test)]
mod tests {
    #[test_case]
    fn dummy_test() {
        use crate::bios;
        use crate::hal::{Register, GPUSTAT};
        assert!(bios::gpu_get_status() == GPUSTAT::load().bits());
    }
}
