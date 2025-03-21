use crate::devices::{CommandPorts, MovementPorts, RadioPorts, UnitIO};
use crate::tools::assembler::{assemble, Program};
use bevy::prelude::*;
use raven_uxn::{Backend, Uxn, UxnRam};
use std::collections::BTreeSet;
use std::path::Path;

use crate::radio::RadioMessage;

pub struct CpuLimits {
    num_cycles: u32,
}

#[derive(Component)]
pub struct Executable {
    pub cpu: Uxn<'static>,
    pub device: UnitIO,
    pub limits: CpuLimits,
    pub program: Program,
    pub breakpoints: BTreeSet<u16>,
    pub pc: Option<u16>,
    /// Keeps a stack of vectors we need to call
    pub vector_queue: Vec<u16>,
    pub unit_id: u64,
    pub cycles_left: u32,
}

impl Executable {
    pub fn from_file(unit_id: u64, path: impl AsRef<Path>) -> Self {
        let src = std::fs::read_to_string(path).unwrap();
        let program = assemble(src).unwrap();
        Executable::from_program(unit_id, &program)
    }

    pub fn from_program(unit_id: u64, program: &Program) -> Self {
        let ram = UxnRam::new();
        let mut uxn = Uxn::new(ram.leak(), Backend::Interpreter);
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
            program: program.clone(),
            breakpoints: BTreeSet::new(),
            pc: None,
            vector_queue: Vec::new(),
            unit_id,
            cycles_left: 0,
        }
    }

    pub fn load_program(&mut self, program: &Program, transform: &mut Transform) {
        let _ = self.cpu.reset(&program.rom);
        self.program = program.clone();
        let mut dev = self.device.arm(transform);
        self.cpu.run(&mut dev, 0x100);
    }

    pub fn add_breakpoint(&mut self, addr: &u16) {
        self.breakpoints.insert(*addr);
    }

    pub fn has_breakpoint_at(&self, addr: &u16) -> bool {
        self.breakpoints.contains(addr)
    }

    pub fn remove_breakpoint(&mut self, addr: &u16) {
        self.breakpoints.remove(addr);
    }

    pub fn step(&mut self, transform: &mut Transform) {
        let mut device = self.device.arm(transform);
        if let Some(pc) = self.pc {
            self.cycles_left -= 1;
            self.pc = self.cpu.step(&mut device, pc);
        }
    }

    pub fn cont(&mut self, transform: &mut Transform) -> Option<RadioMessage> {
        let mut radio_message = None;

        while let Some(pc) = self.pc {
            if self.has_breakpoint_at(&pc) {
                break;
            }
            if self.cycles_left == 0 {
                break;
            }

            let mut device = self.device.arm(transform);
            self.pc = self.cpu.step(&mut device, pc);

            if let Some(rm) = device.radio_message {
                radio_message = Some(rm);
            }
        }

        radio_message
    }

    pub fn start(&mut self) {
        self.pc = Some(0x100);
    }

    pub fn can_step(&self) -> bool {
        self.pc.is_some()
    }

    pub fn execute(&mut self, transform: &mut Transform, vector: u16) {
        let mut device = self.device.arm(transform);
        self.cpu.run(&mut device, vector);
    }

    pub fn move_vector(&mut self) -> u16 {
        let v = self.cpu.dev::<CommandPorts>();
        v.move_vector.get()
    }

    pub fn target_pos(&mut self) -> Vec3 {
        let m = self.cpu.dev::<MovementPorts>();
        Vec3::new(m.tx.get() as f32, m.ty.get() as f32, 0.0)
    }

    pub fn set_current_pos(&mut self, pos: Vec3) {
        let m = self.cpu.dev_mut::<MovementPorts>();
        m.x.set(pos.x as u16);
        m.y.set(pos.y as u16);
    }

    pub fn radio_message_vector(&mut self) -> u16 {
        let v = self.cpu.dev::<RadioPorts>();
        v.vector.get()
    }

    pub fn radio_frequency(&mut self) -> u8 {
        let v = self.cpu.dev::<RadioPorts>();
        v.freq
    }

    pub fn set_radio_packets(&mut self, packets: &[u16; 2]) {
        let v = self.cpu.dev_mut::<RadioPorts>();
        v.packeth.set(packets[0]);
        v.packetl.set(packets[1]);
    }

    pub fn set_move_command_coords(&mut self, x: u16, y: u16) {
        let v = self.cpu.dev_mut::<MovementPorts>();
        v.x.set(x);
        v.y.set(y);
    }

    pub fn loop_vector(&mut self) -> u16 {
        let v = self.cpu.dev::<CommandPorts>();
        v.loop_vector.get()
    }

    fn arbitrary_transform(&self) -> Transform {
        Transform::default()
    }
}
