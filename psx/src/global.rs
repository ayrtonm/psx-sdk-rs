use crate::framebuffer::Framebuffer;
use crate::gpu::{Color, Vertex};
use crate::printer::{Printer, MIN_SIZE};

/// An uninitialized global framebuffer.
pub static mut FRAMEBUFFER: Framebuffer = Framebuffer::new_const(
    Vertex::new(0, 0),
    Vertex::new(0, 240),
    Vertex::new(320, 240),
    Some(Color::BLACK),
);

/// Resets the graphics system and initializes the global framebuffer.
#[macro_export]
macro_rules! init_graphics {
    () => {{
        let gpu_dma = unsafe { &mut $crate::dma::gpu::CHCR::new() };
        $crate::general::reset_graphics(gpu_dma);
        unsafe { $crate::global::FRAMEBUFFER.init(gpu_dma) }
        $crate::general::enable_display();
    }};
}

/// Swaps the global framebuffer.
#[macro_export]
macro_rules! swap {
    () => {
        unsafe {
            $crate::global::FRAMEBUFFER.swap(&mut $crate::dma::gpu::CHCR::new());
        }
    };
}

/// An uninitialized global printer.
pub static mut PRINTER: Printer<MIN_SIZE> = Printer::new_const(
    Vertex::new(0, 0),
    Vertex::new(0, 0),
    Vertex::new(320, 240),
    None,
);

/// Initializes the printer by loading the font into VRAM.
#[macro_export]
macro_rules! load_font {
    () => {
        unsafe {
            $crate::global::PRINTER.load_font(&mut $crate::dma::gpu::CHCR::new());
        }
    };
}

/// Prints a message using the global printer.
#[macro_export]
macro_rules! print {
    ($msg:expr $(,$args:expr)*) => {
        unsafe {
            $crate::global::PRINTER
                .print($msg, [$($args),*], &mut $crate::dma::gpu::CHCR::new());
        }
    };
}

/// Prints a message terminated with a newline using the global printer.
#[macro_export]
macro_rules! println {
    ($msg:expr $(,$args:expr)*) => {
        unsafe {
            $crate::print!($msg $(,$args)*);
            $crate::global::PRINTER.newline();
        }
    };
}
