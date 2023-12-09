use super::super::bus::Bus;
use super::riscv::exception::RvException;

pub trait ArchInterface {
    fn step(&mut self, bus: &mut Bus) -> Option<RvException>;
}
