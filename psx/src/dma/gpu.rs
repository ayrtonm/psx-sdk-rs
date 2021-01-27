use super::GPU;

impl GPU {
    //pub fn send_list<'l, L: LinkedList<'l>, F, R>(&mut self, linked_list: &'l L,
    // f: F) -> R where F: FnOnce(L::Swapped) -> R {
    //    GP1.dma_mode(Some(DMAMode::GP0));
    //    self.madr.set_address(linked_list.address()).store();
    //    let other = linked_list.peek();
    //    self.chcr
    //        .set_direction(Direction::FromMemory)
    //        .set_mode(TransferMode::LinkedList)
    //        .start()
    //        .store();
    //    let r = f(other);
    //    self.chcr.wait();
    //    r
    //}
}
