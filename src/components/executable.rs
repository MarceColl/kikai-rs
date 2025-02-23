use std::path::Path;
use std::io::Read;
use bevy::prelude::*;
use raven_uxn::{Uxn, UxnRam, Backend};
use crate::devices::{UnitIO, CommandPorts, MovementPorts};
use crate::tools::assembler::{assemble, Program};
use anyhow::{Context, Result};

pub struct CpuLimits {
    num_cycles: u32,
}

#[derive(Component)]
pub struct Executable {
    pub cpu: Uxn<'static>,
    pub device: UnitIO,
    pub limits: CpuLimits,
    pub program: Program,
}

impl Executable {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let ram = UxnRam::new();
        let mut uxn = Uxn::new(ram.leak(), Backend::Interpreter);
        let src = std::fs::read_to_string("basic.tal").unwrap();
        let program = assemble(src).unwrap();
        uxn.reset(&program.rom);

        let mut transform = Transform {
            translation: Vec3::new(0., 0., 0.),
            ..default()
        };
        let mut device = UnitIO::new();
        let mut dev = device.arm(&mut transform);
        // Initialize the system
        uxn.run(&mut dev, 0x100);

        let limits = CpuLimits { num_cycles: 10000 };

        Executable {
            cpu: uxn,
            device,
            limits,
            program: program,
        }
    }

    pub fn execute(&mut self, transform: &mut Transform, vector: u16) {
        let mut device = self.device.arm(transform);
        self.cpu.run(&mut device, vector);
    }

    pub fn move_vector(&mut self) -> u16 {
        let mut t = self.arbitrary_transform();
        let mut device = self.device.arm(&mut t);
        let v = self.cpu.dev::<CommandPorts>();
        v.move_vector.get()
    }

    pub fn set_move_command_coords(&mut self, x: u16, y: u16) {
        let mut t = self.arbitrary_transform();
        let mut device = self.device.arm(&mut t);
        let mut v = self.cpu.dev_mut::<MovementPorts>();
        v.x.set(x);
        v.y.set(y);
    }

    pub fn loop_vector(&mut self) -> u16 {
        let mut t = self.arbitrary_transform();
        let mut device = self.device.arm(&mut t);
        let v = self.cpu.dev::<CommandPorts>();
        v.loop_vector.get()
    }

    fn arbitrary_transform(&self) -> Transform {
        Transform::default()
    }
}
