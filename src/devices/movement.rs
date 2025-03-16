use bevy::prelude::*;
use raven_uxn::{Ports, Uxn, DEV_SIZE};
use zerocopy::{BigEndian, U16};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct MovementPorts {
    // |10 @Movement &vector $2 &x $2 &y $2 &tx $2 &ty $2
    pub vector: U16<BigEndian>,
    pub x: U16<BigEndian>,
    pub y: U16<BigEndian>,
    pub tx: U16<BigEndian>,
    pub ty: U16<BigEndian>,
    _p1: u16,
    _p2: u32,
}

impl MovementPorts {
    fn dev<'a>(vm: &'a Uxn, i: usize) -> &'a Self {
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

    pub fn deo(&mut self, _vm: &mut Uxn, _target: u8, _transform: &mut Transform) {}
}
