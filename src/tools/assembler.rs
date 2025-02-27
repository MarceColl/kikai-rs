use crate::egui::{Color32, RichText, WidgetText};
use std::collections::BTreeMap;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Instr {
    BRK,
    INC,
    POP,
    NIP,
    SWP,
    ROT,
    DUP,
    OVR,
    EQU,
    NEQ,
    GTH,
    LTH,
    JMP,
    JCN,
    JSR,
    STH,
    LDZ,
    STZ,
    LDR,
    STR,
    LDA,
    STA,
    DEI,
    DEO,
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    ORA,
    EOR,
    SFT,
    JCI,
    INC2,
    POP2,
    NIP2,
    SWP2,
    ROT2,
    DUP2,
    OVR2,
    EQU2,
    NEQ2,
    GTH2,
    LTH2,
    JMP2,
    JCN2,
    JSR2,
    STH2,
    LDZ2,
    STZ2,
    LDR2,
    STR2,
    LDA2,
    STA2,
    DEI2,
    DEO2,
    ADD2,
    SUB2,
    MUL2,
    DIV2,
    AND2,
    ORA2,
    EOR2,
    SFT2,
    JMI,
    INCr,
    POPr,
    NIPr,
    SWPr,
    ROTr,
    DUPr,
    OVRr,
    EQUr,
    NEQr,
    GTHr,
    LTHr,
    JMPr,
    JCNr,
    JSRr,
    STHr,
    LDZr,
    STZr,
    LDRr,
    STRr,
    LDAr,
    STAr,
    DEIr,
    DEOr,
    ADDr,
    SUBr,
    MULr,
    DIVr,
    ANDr,
    ORAr,
    EORr,
    SFTr,
    JSI,
    INC2r,
    POP2r,
    NIP2r,
    SWP2r,
    ROT2r,
    DUP2r,
    OVR2r,
    EQU2r,
    NEQ2r,
    GTH2r,
    LTH2r,
    JMP2r,
    JCN2r,
    JSR2r,
    STH2r,
    LDZ2r,
    STZ2r,
    LDR2r,
    STR2r,
    LDA2r,
    STA2r,
    DEI2r,
    DEO2r,
    ADD2r,
    SUB2r,
    MUL2r,
    DIV2r,
    AND2r,
    ORA2r,
    EOR2r,
    SFT2r,
    LIT,
    INCk,
    POPk,
    NIPk,
    SWPk,
    ROTk,
    DUPk,
    OVRk,
    EQUk,
    NEQk,
    GTHk,
    LTHk,
    JMPk,
    JCNk,
    JSRk,
    STHk,
    LDZk,
    STZk,
    LDRk,
    STRk,
    LDAk,
    STAk,
    DEIk,
    DEOk,
    ADDk,
    SUBk,
    MULk,
    DIVk,
    ANDk,
    ORAk,
    EORk,
    SFTk,
    LIT2,
    INC2k,
    POP2k,
    NIP2k,
    SWP2k,
    ROT2k,
    DUP2k,
    OVR2k,
    EQU2k,
    NEQ2k,
    GTH2k,
    LTH2k,
    JMP2k,
    JCN2k,
    JSR2k,
    STH2k,
    LDZ2k,
    STZ2k,
    LDR2k,
    STR2k,
    LDA2k,
    STA2k,
    DEI2k,
    DEO2k,
    ADD2k,
    SUB2k,
    MUL2k,
    DIV2k,
    AND2k,
    ORA2k,
    EOR2k,
    SFT2k,
    LITr,
    INCkr,
    POPkr,
    NIPkr,
    SWPkr,
    ROTkr,
    DUPkr,
    OVRkr,
    EQUkr,
    NEQkr,
    GTHkr,
    LTHkr,
    JMPkr,
    JCNkr,
    JSRkr,
    STHkr,
    LDZkr,
    STZkr,
    LDRkr,
    STRkr,
    LDAkr,
    STAkr,
    DEIkr,
    DEOkr,
    ADDkr,
    SUBkr,
    MULkr,
    DIVkr,
    ANDkr,
    ORAkr,
    EORkr,
    SFTkr,
    LIT2r,
    INC2kr,
    POP2kr,
    NIP2kr,
    SWP2kr,
    ROT2kr,
    DUP2kr,
    OVR2kr,
    EQU2kr,
    NEQ2kr,
    GTH2kr,
    LTH2kr,
    JMP2kr,
    JCN2kr,
    JSR2kr,
    STH2kr,
    LDZ2kr,
    STZ2kr,
    LDR2kr,
    STR2kr,
    LDA2kr,
    STA2kr,
    DEI2kr,
    DEO2kr,
    ADD2kr,
    SUB2kr,
    MUL2kr,
    DIV2kr,
    AND2kr,
    ORA2kr,
    EOR2kr,
    SFT2kr,
}

impl From<u8> for Instr {
    fn from(value: u8) -> Self {
        // Since it's exhaustive, we can safely transmute
        unsafe { std::mem::transmute(value) }
    }
}

// Convert from MyEnum to u8
impl From<Instr> for u8 {
    fn from(value: Instr) -> Self {
        value as u8
    }
}

#[derive(Debug)]
pub enum Atom {
    Instr(Instr),
    LParen,
    RParen,
    LBracket,
    RBracket,
    AbsoluteLabel(String),
    RelativeLabel(String),
    DevicePadding(u8),
    AbsolutePadding(u16),
    RelativePadding(u16),
    ByteLiteral(u8),
    ShortLiteral(u16),
    LiteralAbsoluteAddressing(String),
    LiteralZeroPageAddressing(String),
    LiteralRelativeAddressing(String),
    RawAbsoluteAddressing(String),
    RawZeroPageAddressing(String),
    RawRelativeAddressing(String),
    StringLiteral(String),
    ProcCall(String),
    ByteRaw(u8),
    ImmediateJCI(String),
    ImmediateJMI(String),
}

pub struct Span {
    atom: Atom,
    src_string: String,
    start: usize,
    end: usize,
}

struct Parser {
    cursor: usize,
    curr_addr: u16,
}

struct Lexer {
    string: String,
    cursor: usize,
}

fn parse_instruction(chunk: &String) -> Option<Instr> {
    match chunk.as_str() {
        "BRK" => Some(Instr::BRK),
        "INC" => Some(Instr::INC),
        "POP" => Some(Instr::POP),
        "NIP" => Some(Instr::NIP),
        "SWP" => Some(Instr::SWP),
        "ROT" => Some(Instr::ROT),
        "DUP" => Some(Instr::DUP),
        "OVR" => Some(Instr::OVR),
        "EQU" => Some(Instr::EQU),
        "NEQ" => Some(Instr::NEQ),
        "GTH" => Some(Instr::GTH),
        "LTH" => Some(Instr::LTH),
        "JMP" => Some(Instr::JMP),
        "JCN" => Some(Instr::JCN),
        "JSR" => Some(Instr::JSR),
        "STH" => Some(Instr::STH),
        "LDZ" => Some(Instr::LDZ),
        "STZ" => Some(Instr::STZ),
        "LDR" => Some(Instr::LDR),
        "STR" => Some(Instr::STR),
        "LDA" => Some(Instr::LDA),
        "STA" => Some(Instr::STA),
        "DEI" => Some(Instr::DEI),
        "DEO" => Some(Instr::DEO),
        "ADD" => Some(Instr::ADD),
        "SUB" => Some(Instr::SUB),
        "MUL" => Some(Instr::MUL),
        "DIV" => Some(Instr::DIV),
        "AND" => Some(Instr::AND),
        "ORA" => Some(Instr::ORA),
        "EOR" => Some(Instr::EOR),
        "SFT" => Some(Instr::SFT),
        "JCI" => Some(Instr::JCI),
        "INC2" => Some(Instr::INC2),
        "POP2" => Some(Instr::POP2),
        "NIP2" => Some(Instr::NIP2),
        "SWP2" => Some(Instr::SWP2),
        "ROT2" => Some(Instr::ROT2),
        "DUP2" => Some(Instr::DUP2),
        "OVR2" => Some(Instr::OVR2),
        "EQU2" => Some(Instr::EQU2),
        "NEQ2" => Some(Instr::NEQ2),
        "GTH2" => Some(Instr::GTH2),
        "LTH2" => Some(Instr::LTH2),
        "JMP2" => Some(Instr::JMP2),
        "JCN2" => Some(Instr::JCN2),
        "JSR2" => Some(Instr::JSR2),
        "STH2" => Some(Instr::STH2),
        "LDZ2" => Some(Instr::LDZ2),
        "STZ2" => Some(Instr::STZ2),
        "LDR2" => Some(Instr::LDR2),
        "STR2" => Some(Instr::STR2),
        "LDA2" => Some(Instr::LDA2),
        "STA2" => Some(Instr::STA2),
        "DEI2" => Some(Instr::DEI2),
        "DEO2" => Some(Instr::DEO2),
        "ADD2" => Some(Instr::ADD2),
        "SUB2" => Some(Instr::SUB2),
        "MUL2" => Some(Instr::MUL2),
        "DIV2" => Some(Instr::DIV2),
        "AND2" => Some(Instr::AND2),
        "ORA2" => Some(Instr::ORA2),
        "EOR2" => Some(Instr::EOR2),
        "SFT2" => Some(Instr::SFT2),
        "JMI" => Some(Instr::JMI),
        "INCr" => Some(Instr::INCr),
        "POPr" => Some(Instr::POPr),
        "NIPr" => Some(Instr::NIPr),
        "SWPr" => Some(Instr::SWPr),
        "ROTr" => Some(Instr::ROTr),
        "DUPr" => Some(Instr::DUPr),
        "OVRr" => Some(Instr::OVRr),
        "EQUr" => Some(Instr::EQUr),
        "NEQr" => Some(Instr::NEQr),
        "GTHr" => Some(Instr::GTHr),
        "LTHr" => Some(Instr::LTHr),
        "JMPr" => Some(Instr::JMPr),
        "JCNr" => Some(Instr::JCNr),
        "JSRr" => Some(Instr::JSRr),
        "STHr" => Some(Instr::STHr),
        "LDZr" => Some(Instr::LDZr),
        "STZr" => Some(Instr::STZr),
        "LDRr" => Some(Instr::LDRr),
        "STRr" => Some(Instr::STRr),
        "LDAr" => Some(Instr::LDAr),
        "STAr" => Some(Instr::STAr),
        "DEIr" => Some(Instr::DEIr),
        "DEOr" => Some(Instr::DEOr),
        "ADDr" => Some(Instr::ADDr),
        "SUBr" => Some(Instr::SUBr),
        "MULr" => Some(Instr::MULr),
        "DIVr" => Some(Instr::DIVr),
        "ANDr" => Some(Instr::ANDr),
        "ORAr" => Some(Instr::ORAr),
        "EORr" => Some(Instr::EORr),
        "SFTr" => Some(Instr::SFTr),
        "JSI" => Some(Instr::JSI),
        "INC2r" => Some(Instr::INC2r),
        "POP2r" => Some(Instr::POP2r),
        "NIP2r" => Some(Instr::NIP2r),
        "SWP2r" => Some(Instr::SWP2r),
        "ROT2r" => Some(Instr::ROT2r),
        "DUP2r" => Some(Instr::DUP2r),
        "OVR2r" => Some(Instr::OVR2r),
        "EQU2r" => Some(Instr::EQU2r),
        "NEQ2r" => Some(Instr::NEQ2r),
        "GTH2r" => Some(Instr::GTH2r),
        "LTH2r" => Some(Instr::LTH2r),
        "JMP2r" => Some(Instr::JMP2r),
        "JCN2r" => Some(Instr::JCN2r),
        "JSR2r" => Some(Instr::JSR2r),
        "STH2r" => Some(Instr::STH2r),
        "LDZ2r" => Some(Instr::LDZ2r),
        "STZ2r" => Some(Instr::STZ2r),
        "LDR2r" => Some(Instr::LDR2r),
        "STR2r" => Some(Instr::STR2r),
        "LDA2r" => Some(Instr::LDA2r),
        "STA2r" => Some(Instr::STA2r),
        "DEI2r" => Some(Instr::DEI2r),
        "DEO2r" => Some(Instr::DEO2r),
        "ADD2r" => Some(Instr::ADD2r),
        "SUB2r" => Some(Instr::SUB2r),
        "MUL2r" => Some(Instr::MUL2r),
        "DIV2r" => Some(Instr::DIV2r),
        "AND2r" => Some(Instr::AND2r),
        "ORA2r" => Some(Instr::ORA2r),
        "EOR2r" => Some(Instr::EOR2r),
        "SFT2r" => Some(Instr::SFT2r),
        "LIT" => Some(Instr::LIT),
        "INCk" => Some(Instr::INCk),
        "POPk" => Some(Instr::POPk),
        "NIPk" => Some(Instr::NIPk),
        "SWPk" => Some(Instr::SWPk),
        "ROTk" => Some(Instr::ROTk),
        "DUPk" => Some(Instr::DUPk),
        "OVRk" => Some(Instr::OVRk),
        "EQUk" => Some(Instr::EQUk),
        "NEQk" => Some(Instr::NEQk),
        "GTHk" => Some(Instr::GTHk),
        "LTHk" => Some(Instr::LTHk),
        "JMPk" => Some(Instr::JMPk),
        "JCNk" => Some(Instr::JCNk),
        "JSRk" => Some(Instr::JSRk),
        "STHk" => Some(Instr::STHk),
        "LDZk" => Some(Instr::LDZk),
        "STZk" => Some(Instr::STZk),
        "LDRk" => Some(Instr::LDRk),
        "STRk" => Some(Instr::STRk),
        "LDAk" => Some(Instr::LDAk),
        "STAk" => Some(Instr::STAk),
        "DEIk" => Some(Instr::DEIk),
        "DEOk" => Some(Instr::DEOk),
        "ADDk" => Some(Instr::ADDk),
        "SUBk" => Some(Instr::SUBk),
        "MULk" => Some(Instr::MULk),
        "DIVk" => Some(Instr::DIVk),
        "ANDk" => Some(Instr::ANDk),
        "ORAk" => Some(Instr::ORAk),
        "EORk" => Some(Instr::EORk),
        "SFTk" => Some(Instr::SFTk),
        "LIT2" => Some(Instr::LIT2),
        "INC2k" => Some(Instr::INC2k),
        "POP2k" => Some(Instr::POP2k),
        "NIP2k" => Some(Instr::NIP2k),
        "SWP2k" => Some(Instr::SWP2k),
        "ROT2k" => Some(Instr::ROT2k),
        "DUP2k" => Some(Instr::DUP2k),
        "OVR2k" => Some(Instr::OVR2k),
        "EQU2k" => Some(Instr::EQU2k),
        "NEQ2k" => Some(Instr::NEQ2k),
        "GTH2k" => Some(Instr::GTH2k),
        "LTH2k" => Some(Instr::LTH2k),
        "JMP2k" => Some(Instr::JMP2k),
        "JCN2k" => Some(Instr::JCN2k),
        "JSR2k" => Some(Instr::JSR2k),
        "STH2k" => Some(Instr::STH2k),
        "LDZ2k" => Some(Instr::LDZ2k),
        "STZ2k" => Some(Instr::STZ2k),
        "LDR2k" => Some(Instr::LDR2k),
        "STR2k" => Some(Instr::STR2k),
        "LDA2k" => Some(Instr::LDA2k),
        "STA2k" => Some(Instr::STA2k),
        "DEI2k" => Some(Instr::DEI2k),
        "DEO2k" => Some(Instr::DEO2k),
        "ADD2k" => Some(Instr::ADD2k),
        "SUB2k" => Some(Instr::SUB2k),
        "MUL2k" => Some(Instr::MUL2k),
        "DIV2k" => Some(Instr::DIV2k),
        "AND2k" => Some(Instr::AND2k),
        "ORA2k" => Some(Instr::ORA2k),
        "EOR2k" => Some(Instr::EOR2k),
        "SFT2k" => Some(Instr::SFT2k),
        "LITr" => Some(Instr::LITr),
        "INCkr" => Some(Instr::INCkr),
        "POPkr" => Some(Instr::POPkr),
        "NIPkr" => Some(Instr::NIPkr),
        "SWPkr" => Some(Instr::SWPkr),
        "ROTkr" => Some(Instr::ROTkr),
        "DUPkr" => Some(Instr::DUPkr),
        "OVRkr" => Some(Instr::OVRkr),
        "EQUkr" => Some(Instr::EQUkr),
        "NEQkr" => Some(Instr::NEQkr),
        "GTHkr" => Some(Instr::GTHkr),
        "LTHkr" => Some(Instr::LTHkr),
        "JMPkr" => Some(Instr::JMPkr),
        "JCNkr" => Some(Instr::JCNkr),
        "JSRkr" => Some(Instr::JSRkr),
        "STHkr" => Some(Instr::STHkr),
        "LDZkr" => Some(Instr::LDZkr),
        "STZkr" => Some(Instr::STZkr),
        "LDRkr" => Some(Instr::LDRkr),
        "STRkr" => Some(Instr::STRkr),
        "LDAkr" => Some(Instr::LDAkr),
        "STAkr" => Some(Instr::STAkr),
        "DEIkr" => Some(Instr::DEIkr),
        "DEOkr" => Some(Instr::DEOkr),
        "ADDkr" => Some(Instr::ADDkr),
        "SUBkr" => Some(Instr::SUBkr),
        "MULkr" => Some(Instr::MULkr),
        "DIVkr" => Some(Instr::DIVkr),
        "ANDkr" => Some(Instr::ANDkr),
        "ORAkr" => Some(Instr::ORAkr),
        "EORkr" => Some(Instr::EORkr),
        "SFTkr" => Some(Instr::SFTkr),
        "LIT2r" => Some(Instr::LIT2r),
        "INC2kr" => Some(Instr::INC2kr),
        "POP2kr" => Some(Instr::POP2kr),
        "NIP2kr" => Some(Instr::NIP2kr),
        "SWP2kr" => Some(Instr::SWP2kr),
        "ROT2kr" => Some(Instr::ROT2kr),
        "DUP2kr" => Some(Instr::DUP2kr),
        "OVR2kr" => Some(Instr::OVR2kr),
        "EQU2kr" => Some(Instr::EQU2kr),
        "NEQ2kr" => Some(Instr::NEQ2kr),
        "GTH2kr" => Some(Instr::GTH2kr),
        "LTH2kr" => Some(Instr::LTH2kr),
        "JMP2kr" => Some(Instr::JMP2kr),
        "JCN2kr" => Some(Instr::JCN2kr),
        "JSR2kr" => Some(Instr::JSR2kr),
        "STH2kr" => Some(Instr::STH2kr),
        "LDZ2kr" => Some(Instr::LDZ2kr),
        "STZ2kr" => Some(Instr::STZ2kr),
        "LDR2kr" => Some(Instr::LDR2kr),
        "STR2kr" => Some(Instr::STR2kr),
        "LDA2kr" => Some(Instr::LDA2kr),
        "STA2kr" => Some(Instr::STA2kr),
        "DEI2kr" => Some(Instr::DEI2kr),
        "DEO2kr" => Some(Instr::DEO2kr),
        "ADD2kr" => Some(Instr::ADD2kr),
        "SUB2kr" => Some(Instr::SUB2kr),
        "MUL2kr" => Some(Instr::MUL2kr),
        "DIV2kr" => Some(Instr::DIV2kr),
        "AND2kr" => Some(Instr::AND2kr),
        "ORA2kr" => Some(Instr::ORA2kr),
        "EOR2kr" => Some(Instr::EOR2kr),
        "SFT2kr" => Some(Instr::SFT2kr),
        _ => None,
    }
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer {
            string: src,
            cursor: 0,
        }
    }

    pub fn next_span(&mut self) -> Option<Span> {
        if self.cursor >= self.string.len() {
            None
        } else {
            while self
                .string
                .chars()
                .nth(self.cursor)
                .unwrap()
                .is_whitespace()
            {
                self.cursor += 1;
            }

            let src_string: String = self.string[self.cursor..]
                .chars()
                .take_while(|c| !c.is_whitespace())
                .collect();

            let start = self.cursor;
            let end = self.cursor + src_string.len();

            self.cursor = end + 1;

            println!("{} ({}-{})", src_string, start, end);

            let atom = self.lex(&src_string);

            Some(Span {
                atom,
                src_string,
                start: self.cursor,
                end: self.cursor + 4,
            })
        }
    }

    pub fn lex(&mut self, chunk: &String) -> Atom {
        if chunk.starts_with('#') {
            if chunk[1..].len() == 2 {
                Atom::ByteLiteral(u8::from_str_radix(&chunk[1..], 16).unwrap())
            } else if chunk[1..].len() == 4 {
                Atom::ShortLiteral(u16::from_str_radix(&chunk[1..], 16).unwrap())
            } else {
                panic!("Wrong Literal");
            }
        } else if chunk.starts_with('|') {
            Atom::AbsolutePadding(u16::from_str_radix(&chunk[1..], 16).unwrap())
        } else if chunk.starts_with('$') {
            Atom::RelativePadding(u16::from_str_radix(&chunk[1..], 16).unwrap())
        } else if chunk.starts_with('?') {
            Atom::ImmediateJCI(chunk[2..].to_string())
        } else if chunk.starts_with('"') {
            Atom::StringLiteral(chunk[1..].to_string())
        } else if chunk.starts_with('@') {
            Atom::AbsoluteLabel(chunk[1..].to_string())
        } else if chunk.starts_with('&') {
            Atom::RelativeLabel(chunk[1..].to_string())
        } else if chunk.starts_with(';') {
            Atom::LiteralAbsoluteAddressing(chunk[1..].to_string())
        } else if chunk.starts_with('.') {
            Atom::LiteralZeroPageAddressing(chunk[1..].to_string())
        } else if chunk.starts_with(',') {
            Atom::LiteralRelativeAddressing(chunk[1..].to_string())
        } else if chunk.starts_with('=') {
            Atom::RawAbsoluteAddressing(chunk[1..].to_string())
        } else if chunk.starts_with('-') {
            Atom::RawZeroPageAddressing(chunk[1..].to_string())
        } else if chunk.starts_with('_') {
            Atom::RawRelativeAddressing(chunk[1..].to_string())
        } else if chunk.starts_with('(') {
            Atom::LParen
            // THIS IS TERRIBLE:
        } else if chunk.starts_with('[') {
            Atom::LBracket
        } else if chunk.starts_with(']') {
            Atom::RBracket
        } else if chunk.starts_with(')') || chunk.ends_with(')') {
            Atom::RParen
        } else if let Some(instr) = parse_instruction(chunk) {
            Atom::Instr(instr)
        } else if chunk.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            Atom::ByteRaw(u8::from_str_radix(chunk, 16).unwrap())
        } else {
            Atom::ProcCall(chunk.to_string())
        }
    }
}

pub fn rom_size(atom: &Atom) -> u16 {
    match atom {
        Atom::AbsolutePadding(_)
        | Atom::RelativePadding(_)
        | Atom::AbsoluteLabel(_)
        | Atom::RelativeLabel(_)
        | Atom::LParen
        | Atom::RParen
        | Atom::LBracket
        | Atom::RBracket => 0,
        Atom::Instr(_)
        | Atom::ByteRaw(_)
        | Atom::RawZeroPageAddressing(_)
        | Atom::RawRelativeAddressing(_) => 1,
        Atom::ByteLiteral(_)
        | Atom::LiteralZeroPageAddressing(_)
        | Atom::LiteralRelativeAddressing(_)
        | Atom::RawAbsoluteAddressing(_) => 2,
        Atom::LiteralAbsoluteAddressing(_)
        | Atom::ShortLiteral(_)
        | Atom::ProcCall(_)
        | Atom::ImmediateJCI(_) => 3,
        Atom::StringLiteral(string) => string.len() as u16,
        _ => 0,
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub rom: Vec<u8>,
    pub symbol_table: BTreeMap<String, u16>,
}

pub fn assemble(src: String) -> Result<Program, String> {
    let mut lexer = Lexer::new(src);

    let cursor: usize = 0;
    let mut curr_addr: u16 = 0;
    let mut in_comment = false;

    let mut program = Program {
        rom: vec![],
        symbol_table: BTreeMap::new(),
    };

    let mut spans = vec![];
    let mut current_scope = "".to_string();

    while let Some(span) = lexer.next_span() {
        match &span.atom {
            Atom::AbsoluteLabel(label) => {
                program.symbol_table.insert(label.to_string(), curr_addr);
                current_scope = label.to_string();
            }
            Atom::RelativeLabel(label) => {
                let full_label = format!("{}/{}", current_scope, label);
                program.symbol_table.insert(full_label, curr_addr);
            }
            Atom::RelativePadding(pad) => {
                curr_addr += pad;
            }
            Atom::AbsolutePadding(addr) => {
                curr_addr = *addr;
            }
            atom => curr_addr += rom_size(atom),
        }

        spans.push(span);
    }

    println!("{:?}", program.symbol_table);

    let mut current_scope = "".to_string();

    curr_addr = 0;

    for span in spans {
        curr_addr += rom_size(&span.atom);
        match &span.atom {
            Atom::LParen => {
                in_comment = true;
            }
            Atom::RParen => {
                in_comment = false;
            }
            Atom::LBracket | Atom::RBracket => {}
            Atom::Instr(instr) => {
                program.rom.push((*instr).into());
            }
            Atom::AbsolutePadding(addr) => {
                curr_addr = *addr;
            }
            Atom::RelativePadding(pad) => {
                curr_addr += pad;
            }
            Atom::ByteLiteral(literal) => {
                program.rom.push(Instr::LIT.into());
                program.rom.push(*literal);
            }
            Atom::ShortLiteral(literal) => {
                program.rom.push(Instr::LIT2.into());
                let bytes = literal.to_be_bytes();
                program.rom.push(bytes[0]);
                program.rom.push(bytes[1]);
            }
            Atom::AbsoluteLabel(label) => {
                current_scope = label.to_string();
            }
            Atom::RelativeLabel(label) => {}
            Atom::LiteralAbsoluteAddressing(label) => {
                program.rom.push(Instr::LIT2.into());
                let bytes = program.symbol_table[label].to_be_bytes();
                program.rom.push(bytes[0]);
                program.rom.push(bytes[1]);
            }
            Atom::LiteralZeroPageAddressing(label) => {
                program.rom.push(Instr::LIT.into());
                if let Some(lit) = program.symbol_table.get(label) {
                    let bytes = lit.to_be_bytes();
                    program.rom.push(bytes[1]);
                } else {
                    panic!("Couldn't find label {}", label);
                }
            }
            Atom::ByteRaw(byte) => {
                program.rom.push(*byte);
            }
            Atom::ImmediateJCI(label) => {
                let full_label = format!("{}/{}", current_scope, label);
                if let Some(addr) = program.symbol_table.get(&full_label) {
                    program.rom.push(Instr::JCI.into());
                    let rel_move: i16 = (*addr as i16) - (curr_addr as i16);
                    let bytes = rel_move.to_be_bytes();
                    program.rom.push(bytes[0]);
                    program.rom.push(bytes[1]);
                } else {
                    panic!("ImmediateJCI: Couldn't find label {}", full_label);
                }
            }
            Atom::ProcCall(label) => {
                if let Some(addr) = program.symbol_table.get(label.as_str()) {
                    program.rom.push(Instr::JSI.into());
                    let rel_move: i16 = (*addr as i16) - (curr_addr as i16);
                    let bytes = rel_move.to_be_bytes();
                    program.rom.push(bytes[0]);
                    program.rom.push(bytes[1]);
                } else {
                    panic!("ProcCall: Couldn't find label {}", label);
                }
            }
            Atom::StringLiteral(text) => {
                if !text.is_ascii() {
                    panic!("Only ascii supported!");
                }

                for ch in text.chars() {
                    program.rom.push(ch as u8);
                }
            }
            _ => unimplemented!("{:?}", span.atom),
        }
    }

    Ok(program)
}

pub enum DisassmAtom {
    Lit(u8),
    Lit2(u16),
    Jsi(i16),
    Jci(i16),
    Jmi(i16),
    Instr(Instr),
    AbsoluteLabel(String),
    RelativeLabel(String),
}

pub struct DisassmSpan {
    pub addr: u16,
    pub atom: DisassmAtom,
}

pub struct Disassm {
    pub spans: Vec<DisassmSpan>,
}

pub fn disassm(program: &Program) -> Disassm {
    let rom = &program.rom;
    let inverse_sym_table: BTreeMap<u16, Vec<String>> = program
        .symbol_table
        .clone()
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, (k, v)| {
            acc.entry(v).or_default().push(k);
            acc
        });
    let mut pointer = 0;
    let base_addr = 0x100;

    let mut spans: Vec<DisassmSpan> = vec![];

    while pointer < rom.len() {
        let instr: Instr = rom[pointer].into();

        let addr: u16 = (base_addr + pointer) as u16;
        if let Some(labels) = inverse_sym_table.get(&addr) {
            for label in labels {
                spans.push(DisassmSpan {
                    atom: DisassmAtom::AbsoluteLabel(label.to_string()),
                    addr,
                });
            }
        }

        match instr {
            Instr::LIT => {
                let d = rom[pointer + 1];
                spans.push(DisassmSpan {
                    atom: DisassmAtom::Lit(d),
                    addr,
                });
                pointer += 2;
            }
            Instr::LIT2 => {
                let d1 = rom[pointer + 1];
                let d2 = rom[pointer + 2];
                let d = u16::from_be_bytes([d1, d2]);
                spans.push(DisassmSpan {
                    atom: DisassmAtom::Lit2(d),
                    addr,
                });
                pointer += 3;
            }
            Instr::JSI => {
                let d1 = rom[pointer + 1];
                let d2 = rom[pointer + 2];
                let d = i16::from_be_bytes([d1, d2]);
                spans.push(DisassmSpan {
                    atom: DisassmAtom::Jsi(d),
                    addr,
                });
                pointer += 3;
            }
            Instr::JCI => {
                let d1 = rom[pointer + 1];
                let d2 = rom[pointer + 2];
                let d = i16::from_be_bytes([d1, d2]);
                spans.push(DisassmSpan {
                    atom: DisassmAtom::Jci(d),
                    addr,
                });
                pointer += 3;
            }
            Instr::JMI => {
                let d1 = rom[pointer + 1];
                let d2 = rom[pointer + 2];
                let d = i16::from_be_bytes([d1, d2]);
                spans.push(DisassmSpan {
                    atom: DisassmAtom::Jmi(d),
                    addr,
                });
                pointer += 3;
            }
            instr => {
                spans.push(DisassmSpan {
                    atom: DisassmAtom::Instr(instr),
                    addr,
                });
                pointer += 1;
            }
        }
    }

    Disassm { spans }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_assemble() {
        let src = "|100 #01 #02 ADD BRK".to_string();
        let program = assemble(src).unwrap();

        assert_eq!(program.symbol_table, BTreeMap::new());
        assert_eq!(program.rom, vec![0x80, 0x01, 0x80, 0x02, 0x18, 0x00],)
    }

    #[test]
    fn test_basic_assemble_2() {
        // 0000000 01a0 6012 0100 9400 1880 2117 2094 f7ff
        // 0000010 6c22 6548 6c6c 206f 6f57 6c72 2164
        let src = "|10 @Console &vector $2 &read $1 &pad $5 &write $1 &error $1

|100

@on-reset
	;my-string print-text
	BRK

@print-text
	&while
		LDAk .Console/write DEO
		INC2 LDAk ?&while
	POP2
	JMP2r

@my-string
	\"Hello 20 \"World! 00
"
        .to_string();
        let program = assemble(src).unwrap();
        let expected = vec![
            0xa0, 0x01, 0x12, 0x60, 0x00, 0x01, 0x00, 0x94, 0x80, 0x18, 0x17, 0x21, 0x94, 0x20,
            0xff, 0xf7, 0x22, 0x6c, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c,
            0x64, 0x21, 0x00,
        ];
        let expected_program = Program {
            rom: expected.clone(),
            symbol_table: BTreeMap::new(),
        };

        assert_eq!(program.rom, expected)
    }
}
