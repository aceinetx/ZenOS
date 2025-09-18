use alloc::fmt::Display;
use alloc::string::String;
use alloc::vec::Vec;
use bincode::*;
use uefi_services::*;

#[derive(Encode, Decode, Debug)]
pub enum BlockValue {
    None,
    Number(f64),
    String(String),
    Boolean(bool),
    VarRef(String),
}

#[derive(Encode, Decode, Debug)]
pub enum Value {
    None,
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let result;
        match self {
            Value::None => {
                result = write!(f, "None");
            }
            Value::Number(n) => {
                result = write!(f, "{}", n);
            }
            Value::String(s) => {
                result = write!(f, "{}", s);
            }
            Value::Boolean(flag) => {
                result = write!(f, "{}", flag);
            }
        }
        return result;
    }
}

#[derive(Encode, Decode, Debug)]
pub enum BlockKind {
    Function { name: String },
    Return(BlockValue),
    BasicBlock(),
}

#[derive(Encode, Decode, Debug)]
pub struct Block {
    kind: BlockKind,
    parent: Option<usize>,
    children: Vec<usize>,
    id: usize,
}

#[derive(Encode, Decode, Debug)]
pub struct BlockArena {
    blocks: Vec<Block>,
}

impl BlockArena {
    pub fn new() -> Self {
        return Self { blocks: Vec::new() };
    }

    pub fn add_block(&mut self, kind: BlockKind, parent: Option<usize>) -> usize {
        let idx = self.blocks.len();
        self.blocks.push(Block {
            kind: kind,
            parent: parent,
            children: Vec::new(),
            id: idx,
        });
        if let Some(p) = parent {
            self.blocks[p].children.push(idx);
        }
        return idx;
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        return &self.blocks;
    }

    pub fn get_block(&self, id: usize) -> Result<&Block, &'static str> {
        for block in self.blocks.iter() {
            if block.id == id {
                return Ok(block);
            }
        }
        return Err("could not find block");
    }

    pub fn get_block_mut(&mut self, id: usize) -> Result<&mut Block, &'static str> {
        for block in self.blocks.iter_mut() {
            if block.id == id {
                return Ok(block);
            }
        }
        return Err("could not find block");
    }
}

#[derive(Encode, Decode, Debug)]
pub struct Module {
    pub blocks: BlockArena,
}

impl Module {
    pub fn new() -> Module {
        return Module {
            blocks: BlockArena::new(),
        };
    }

    pub fn compile(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        let cfg = bincode::config::standard();
        let bytes = bincode::encode_to_vec(self, cfg);
        return bytes;
    }

    pub fn load(_bytes: Vec<u8>) {
        todo!();
    }

    pub fn get_block(&self, id: usize) -> Result<&Block, &'static str> {
        return self.blocks.get_block(id);
    }
}

pub struct VM<'a> {
    modules: Vec<&'a mut Module>,
    shared_blocks: BlockArena,
    pc: usize,
    module_pc: usize,
    return_value: Value,
    entry_fn_id: usize,
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        return VM {
            modules: Vec::new(),
            shared_blocks: BlockArena::new(),
            pc: 0,
            module_pc: 0,
            return_value: Value::None,
            entry_fn_id: 0,
        };
    }

    pub fn load_module(&mut self, module: &'a mut Module) {
        self.modules.push(module);
    }

    pub fn get_pc(&self) -> usize {
        return self.pc;
    }

    pub fn get_module_pc(&self) -> usize {
        return self.module_pc;
    }

    pub fn get_return_value(&self) -> &Value {
        return &self.return_value;
    }

    pub fn set_entry_function(&mut self, entry_fn_name: String) -> Result<(), &'static str> {
        for module_id in 0..self.modules.len() {
            let module = self.modules.get_mut(module_id).unwrap();

            for block in module.blocks.get_blocks() {
                if let BlockKind::Function { name: fn_name } = &block.kind {
                    if fn_name.eq(&entry_fn_name) {
                        self.pc = block.id + 1;
                        self.module_pc = module_id;
                        self.entry_fn_id = block.id;
                        return Ok(());
                    }
                }
            }
        }

        return Err("cannot find entry function");
    }

    fn get_value_from_block_value(&self, value: &BlockValue) -> Value {
        match value {
            BlockValue::None => {
                return Value::None;
            }
            BlockValue::Number(n) => {
                return Value::Number(*n);
            }
            BlockValue::String(s) => {
                return Value::String(s.clone());
            }
            BlockValue::Boolean(flag) => {
                return Value::Boolean(*flag);
            }
            BlockValue::VarRef(flag) => {
                todo!();
            }
        }
    }

    pub fn step(&mut self) -> bool {
        if self.module_pc >= self.modules.len() {
            return false;
        }

        if let Ok(block) = self.modules[self.module_pc].get_block(self.pc) {
            println!("executing {:?}", block);
            match &block.kind {
                BlockKind::Return(value) => {
                    self.return_value = self.get_value_from_block_value(value);
                }
                _ => {}
            }
        } else {
            return false;
        }

        self.pc += 1;
        return true;
    }
}
