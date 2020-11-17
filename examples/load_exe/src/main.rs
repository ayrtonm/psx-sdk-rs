#![no_std]
#![no_main]

use psx::bios;

psx::exe!();

fn main(mut ctxt: Ctxt) {
    //let file_name = "ROTATESQ.PSX".as_ptr();
    let file_name: &[u8] = b"CDROM:\\ROTATESQ.PSX\x00";
    let mut buffer = [0; 0x3800];
    let x = bios::file_open(file_name.as_ptr(), 1);
    if x == 0xff {
        let mut draw_port = ctxt.take_draw_port().expect("DrawPort has been taken");
        let c = psx::gpu::vertex::Vertex::new(128, 128);
        draw_port.draw_square(&c, 128, &psx::gpu::color::Color::blue());
        loop {}
    } else {
        bios::load_exe_file(file_name.as_ptr(), buffer.as_mut_ptr());
        bios::do_execute(buffer.as_mut_ptr(), 0, 0);
        //bios::load_and_execute(file_name.as_ptr(), 0x801F_FFF0, 0);
    }
}
