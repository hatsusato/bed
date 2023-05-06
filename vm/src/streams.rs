use crate::memory::Memory;
use crate::reg::Registers;
use util::{Select, Stream, BYTE_COUNT};

#[derive(Clone, Copy)]
struct Descriptor {
    desc: u8,
}
impl Descriptor {
    fn new(desc: u8) -> Self {
        Self { desc }
    }
    const STDIN: Self = Self { desc: 0 };
    const STDOUT: Self = Self { desc: 1 };
    const STDERR: Self = Self { desc: 2 };
}

struct StreamArray {
    array: [Stream; BYTE_COUNT],
}
impl Default for StreamArray {
    fn default() -> Self {
        let array = [(); BYTE_COUNT].map(|_| Stream::Empty);
        Self { array }
    }
}
impl StreamArray {
    fn new() -> Self {
        let mut this = Self::default();
        *this.get_mut(Descriptor::STDIN) = Stream::Stdin;
        *this.get_mut(Descriptor::STDOUT) = Stream::Stdout;
        *this.get_mut(Descriptor::STDERR) = Stream::Stderr;
        this
    }
    fn get_mut(&mut self, desc: Descriptor) -> &mut Stream {
        &mut self.array[usize::from(desc.desc)]
    }
}

pub struct StreamMap {
    array: StreamArray,
    input: Descriptor,
    output: Descriptor,
}
impl StreamMap {
    pub fn new() -> Self {
        Self {
            array: StreamArray::new(),
            input: Descriptor::STDIN,
            output: Descriptor::STDOUT,
        }
    }
    pub fn init(&mut self, input: Stream, output: Stream) {
        *self.select_stream(Select::Input) = input;
        *self.select_stream(Select::Output) = output;
    }
    pub fn getchar(&mut self, regs: &mut Registers, mem: &mut Memory) {
        let flag = self
            .select_stream(Select::Input)
            .getchar()
            .map(|data| mem.putchar(regs, data));
        regs.raise(flag);
    }
    pub fn putchar(&mut self, regs: &mut Registers, mem: &mut Memory) {
        let flag = self
            .select_stream(Select::Output)
            .putchar(mem.getchar(regs));
        regs.raise(flag);
    }
    pub fn stream(&mut self, regs: &mut Registers) {
        match regs.data {
            0 => self.get_descriptor(regs, Select::Input),
            1 => self.get_descriptor(regs, Select::Output),
            2 => self.set_descriptor(regs, Select::Input),
            3 => self.set_descriptor(regs, Select::Output),
            4 => self.argc(regs),
            5 => self.argv(regs),
            6 => self.open_queue(),
            7 => self.open_standard(regs),
            8 => self.open_file(regs),
            _ => (),
        }
    }
    fn open_queue(&mut self) {
        *self.select_stream(Select::Output) = Stream::make_queue();
    }
    fn open_standard(&mut self, regs: &Registers) {
        if let Some(stream) = Stream::make_standard(regs.accum) {
            *self.select_stream(Select::Output) = stream;
        }
    }
    fn open_file(&mut self, regs: &mut Registers) {
        let flag = self
            .select_stream(Select::Input)
            .take_string()
            .map(|path| Stream::make_file(path, util::Flag::new(regs.accum)))
            .and_then(|stream| conditional_option(!matches!(stream, Stream::Empty), || stream))
            .map(|file| *self.select_stream(Select::Output) = file);
        regs.raise(flag);
    }
    fn argc(&mut self, regs: &mut Registers) {
        let flag = self
            .write_number(std::env::args().len())
            .map(|count| regs.accum = count);
        regs.raise(flag);
    }
    fn argv(&mut self, regs: &mut Registers) {
        let flag = self
            .read_number(regs.accum)
            .and_then(|index| std::env::args().nth(index))
            .and_then(|arg| self.select_stream(Select::Output).write(arg.as_bytes()))
            .map(|_| ());
        regs.raise(flag);
    }
    fn read_number(&mut self, count: u8) -> Option<usize> {
        const SIZE: usize = std::mem::size_of::<usize>();
        let count = usize::from(count);
        let mut bytes = [0; SIZE];
        conditional_option(count < SIZE, || &mut bytes[..count])
            .and_then(|buf| self.select_stream(Select::Input).read(buf))
            .and_then(|len| conditional_option(len == count, || bytes))
            .map(usize::from_le_bytes)
    }
    fn write_number(&mut self, number: usize) -> Option<u8> {
        let bytes = number.to_le_bytes();
        let count = bytes.into_iter().rev().skip_while(|&x| x == 0).count();
        self.select_stream(Select::Output)
            .write(&bytes[..count])
            .map(u8::try_from)
            .and_then(util::to_option)
    }
    fn get_descriptor(&self, regs: &mut Registers, select: Select) {
        regs.accum = self.select_descriptor(select).desc;
    }
    fn set_descriptor(&mut self, regs: &mut Registers, select: Select) {
        let desc = Descriptor::new(regs.accum);
        match select {
            Select::Input => self.input = desc,
            Select::Output => self.output = desc,
        }
    }
    fn select_stream(&mut self, select: Select) -> &mut Stream {
        let desc = self.select_descriptor(select);
        self.array.get_mut(desc)
    }
    fn select_descriptor(&self, select: Select) -> Descriptor {
        match select {
            Select::Input => self.input,
            Select::Output => self.output,
        }
    }
}

fn conditional_option<T, F: FnOnce() -> T>(x: bool, f: F) -> Option<T> {
    if x {
        Some(f())
    } else {
        None
    }
}
