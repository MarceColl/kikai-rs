use bevy::prelude::*;
use raven_uxn::{Ports, Uxn, DEV_SIZE};
use zerocopy::{BigEndian, U16};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::radio::RadioMessage;

#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct RadioPorts {
    // |30 @Radio &vector $2 &packeth $2 &packetl $2 &command $1 &freq $1 &strength $1 &enabled $1
    pub vector: U16<BigEndian>,
    pub packeth: U16<BigEndian>,
    pub packetl: U16<BigEndian>,
    pub command: u8,
    pub freq: u8,
    pub strength: u8,
    pub enabled: u8,
    _p2: u16,
    _p1: u32,
}

impl RadioPorts {
    fn dev<'a>(vm: &'a Uxn, i: usize) -> &'a Self {
        let pos = Self::BASE + (i * DEV_SIZE) as u8;
        vm.dev_at(pos)
    }

    fn dev_mut<'a>(vm: &'a mut Uxn, i: usize) -> &'a mut Self {
        let pos = Self::BASE + (i * DEV_SIZE) as u8;
        vm.dev_mut_at(pos)
    }
}

impl Ports for RadioPorts {
    const BASE: u8 = 0x20;
}

pub struct Radio {}

impl Radio {
    pub fn new() -> Self {
        Radio {}
    }

    pub fn deo(&mut self, vm: &mut Uxn, target: u8) -> Option<RadioMessage> {
        let d = vm.dev::<RadioPorts>();
        match target & 0x0F {
            6 => { // Command
                match d.command {
                    0 => {
                        println!("SENT RADIO PACKET AT FREQ {:X}: {:04X}{:04X}", d.freq, d.packeth.get(), d.packetl.get());
                        Some(RadioMessage {
                            origin_entity_id: None,
                            packets: [d.packeth.get(), d.packetl.get()],
                            frequency: d.freq,
                        })
                    },
                    _ => {
                        println!("UNKNOWN COMMAND");
                        None
                    }
                }
            },
            _ => { None },
        }
    }
}
