use std::process::exit;

use crate::stack::Stack;
use spider_vm::bytecode::{Bytecode, Opcode};
use spider_vm::object::Object;

#[derive(Debug, Clone)]
pub struct Locals {
    inner: Vec<Object>,
}

impl Locals {
    pub fn make() -> Self {
        Self { inner: vec![] }
    }

    pub fn get_by_index(&self, index: usize) -> Object {
        if index >= self.inner.len() {
            eprintln!(
                "[Error]: Couldn't access to locals by index {}: OutOfRange",
                index
            );
            exit(1);
        }
        self.inner[index].clone()
    }

    pub fn get_as_ref(&mut self, index: usize) -> &mut Object {
        if index >= self.inner.len() {
            eprintln!(
                "[Error]: Couldn't access to locals by index {}: OutOfRange",
                index
            );
            exit(1);
        }
        &mut self.inner[index]
    }

    pub fn store_at(&mut self, index: usize, o: Object) {
        self.inner[index] = o;
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub pc: usize,
    code: Bytecode,
    pub locals: Locals,
    pub opstack: Stack<Object>,
}

impl Frame {
    pub fn make(code: Bytecode) -> Self {
        Self {
            pc: 0,
            code,
            opstack: Stack::make(),
            locals: Locals::make(),
        }
    }

    pub fn fetch_next_instr(&mut self) -> Opcode {
        let instr = self.code.fetch_by_index(self.pc);
        self.pc += 1;
        instr
    }

    pub fn stack_push(&mut self, o: Object) {
        self.opstack.push(o);
    }

    pub fn stack_pop(&mut self) -> Object {
        self.opstack.pop()
    }
}
