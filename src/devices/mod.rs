use bevy::prelude::*;
use raven_uxn::{Uxn, Device, Ports};

pub mod command;
pub mod movement;

pub use command::{CommandPorts, Command};
pub use movement::{MovementPorts, Movement};

pub struct UnitIO {
    command: Command,
    movement: Movement,
}

pub struct ArmedUnitIO<'a> {
    transform: &'a mut Transform,
    unit_io: &'a mut UnitIO,
}

impl UnitIO {
    pub fn new() -> Self {
        UnitIO {
            command: Command::new(),
            movement: Movement::new(),
        }
    }

    pub fn arm<'a>(&'a mut self, transform: &'a mut Transform) -> ArmedUnitIO<'a> {
        ArmedUnitIO { transform, unit_io: self }
    }
}

impl Device for ArmedUnitIO<'_> {
    fn deo(&mut self, vm: &mut Uxn, target: u8) -> bool {
        match target & 0xF0 {
            CommandPorts::BASE => self.unit_io.command.deo(vm, target),
            MovementPorts::BASE => self.unit_io.movement.deo(vm, target, self.transform),
            _ => { println!("UNIMPLEMENTED DEVICE") }
        };
        true
    }

    fn dei(&mut self, vm: &mut Uxn, target: u8) {
        println!("DEI {}", target);
    }
}
