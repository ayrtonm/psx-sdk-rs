#![no_std]
#![no_main]

use libpsx::bios;

libpsx::exe!();

fn main(mut ctxt: Ctxt) {
    //let file_name = "ROTATESQ.PSX".as_ptr();
    let file_name: &[u8] = b"CDROM:\\ROTATESQ.PSX\x00";
    let mut buffer = [0; 0x3800];
    let x = bios::file_open(file_name.as_ptr(), 1);
    if x == 0xff {
        let mut draw_env = ctxt.take_draw_env().unwrap();
        let c = libpsx::gpu::vertex::Vertex::new(128, 128);
        draw_env.draw_square(&c, 128, &libpsx::gpu::color::Color::blue());
        loop {}
    } else {
        bios::load_exe_file(file_name.as_ptr(), buffer.as_mut_ptr());
        bios::do_execute(buffer.as_mut_ptr(), 0, 0);
        //bios::load_and_execute(file_name.as_ptr(), 0x801F_FFF0, 0);
    }
}
