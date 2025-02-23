
use bevy::prelude::*;
use raven_uxn::{Ports, Uxn, DEV_SIZE};
use zerocopy::{U16, BigEndian};
use zerocopy_derive::{IntoBytes, FromBytes, KnownLayout, Immutable};

#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct MovementPorts {
    // |10 @Movement &vector $2 &x $2 &y $2 &dir $1
    pub vector: U16<BigEndian>,
    pub x: U16<BigEndian>,
    pub y: U16<BigEndian>,
    pub dir: u8,
    _p1: u8,
    _p2: u64
}

impl MovementPorts {
    fn dev<'a>(vm : &'a Uxn, i: usize) -> &'a Self {
        let pos = Self::BASE + (i * DEV_SIZE) as u8;
        vm.dev_at(pos)
    }

    fn dev_mut<'a>(vm: &'a mut Uxn, i: usize) -> &'a mut Self {
        let pos = Self::BASE + (i * DEV_SIZE) as u8;
        vm.dev_mut_at(pos)
    }
}

impl Ports for MovementPorts {
    const BASE: u8 = 0x10;
}

pub struct Movement {}

impl Movement {
    pub fn new() -> Self {
        Movement {}
    }

    pub fn deo(&mut self, vm: &mut Uxn, target: u8, transform: &mut Transform) {
        let d = vm.dev::<MovementPorts>();
        match d.dir % 5 {
            0 => {},
            1 => transform.translation.x += 1.,
            2 => transform.translation.y -= 1.,
            3 => transform.translation.x -= 1.,
            4 => transform.translation.y += 1.,
            _ => { unreachable!(); },
        };
    }
}
