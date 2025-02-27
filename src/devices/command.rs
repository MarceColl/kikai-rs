///! The Command device is the basic command loop interactive device
///! It exposes several vectors that are core to the Kikai control loop
///!
///! move_vector -> Called when the unit is given a move command (maybe move it to a radio device?)
///! attack_vector -> Called when the unit is given an attack command (maybe move it to a radio device?)
///! loop_vector -> This is the main loop vector for repetitive tasks
use raven_uxn::{Ports, Uxn, DEV_SIZE};
use zerocopy::{BigEndian, U16};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct CommandPorts {
    pub move_vector: U16<BigEndian>,
    pub attack_vector: U16<BigEndian>,
    pub create_vector: U16<BigEndian>,
    pub x: U16<BigEndian>,
    pub y: U16<BigEndian>,
    pub loop_vector: U16<BigEndian>,
    _padding: u32,
}

impl Ports for CommandPorts {
    const BASE: u8 = 0x00;
}

pub struct Command {}

impl Command {
    pub fn new() -> Self {
        Self {}
    }

    pub fn deo(&mut self, vm: &mut Uxn, target: u8) {}

    pub fn loop_vector(&mut self, vm: &Uxn) -> u16 {
        vm.dev::<CommandPorts>().loop_vector.get()
    }

    pub fn move_vector(&mut self, vm: &Uxn) -> u16 {
        vm.dev::<CommandPorts>().move_vector.get()
    }
}

impl CommandPorts {
    fn dev<'a>(vm: &'a Uxn, i: usize) -> &'a Self {
        let pos = Self::BASE + (i * DEV_SIZE) as u8;
        vm.dev_at(pos)
    }

    fn dev_mut<'a>(vm: &'a mut Uxn, i: usize) -> &'a mut Self {
        let pos = Self::BASE + (i * DEV_SIZE) as u8;
        vm.dev_mut_at(pos)
    }
}
