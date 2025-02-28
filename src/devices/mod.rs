use bevy::prelude::*;
use raven_uxn::{Device, Ports, Uxn};

pub mod command;
pub mod movement;
pub mod radio;

pub use command::{Command, CommandPorts};
pub use movement::{Movement, MovementPorts};
pub use radio::{Radio, RadioPorts};

use crate::radio::RadioMessage;

pub struct UnitIO {
    command: Command,
    movement: Movement,
    radio: Radio,
}

pub struct ArmedUnitIO<'a> {
    pub transform: &'a mut Transform,
    pub radio_message: Option<RadioMessage>,
    pub unit_io: &'a mut UnitIO,
}

impl UnitIO {
    pub fn new() -> Self {
        UnitIO {
            command: Command::new(),
            movement: Movement::new(),
            radio: Radio::new(),
        }
    }

    pub fn arm<'a>(&'a mut self, transform: &'a mut Transform) -> ArmedUnitIO<'a> {
        ArmedUnitIO {
            transform,
            radio_message: None,
            unit_io: self,
        }
    }
}

impl Device for ArmedUnitIO<'_> {
    fn deo(&mut self, vm: &mut Uxn, target: u8) -> bool {
        println!("TARGET: {}", target & 0xF0);
        match target & 0xF0 {
            CommandPorts::BASE => self.unit_io.command.deo(vm, target),
            MovementPorts::BASE => self.unit_io.movement.deo(vm, target, self.transform),
            RadioPorts::BASE => {
                self.radio_message = self.unit_io.radio.deo(vm, target);
            },
            _ => {
                println!("UNIMPLEMENTED DEVICE")
            }
        };
        true
    }

    fn dei(&mut self, vm: &mut Uxn, target: u8) {
        println!("DEI {}", target);
    }
}
