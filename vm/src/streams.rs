use crate::memory::Memory;
use crate::reg::Registers;
use util::{Select, Stream};

const STREAM_COUNT: usize = 1 << u8::BITS;

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
impl From<Descriptor> for u8 {
    fn from(value: Descriptor) -> Self {
        value.desc
    }
}

struct StreamArray {
    array: [Stream; STREAM_COUNT],
}
impl Default for StreamArray {
    fn default() -> Self {
        let array = [(); STREAM_COUNT].map(|_| Stream::Empty);
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
            _ => unimplemented!(),
        }
    }
    fn argc(&mut self, regs: &mut Registers) {
        let len = std::env::args().len().to_le_bytes();
        let count = len.into_iter().rev().skip_while(|&x| x == 0).count();
        let buf: Vec<_> = len.into_iter().take(count).collect();
        let stream = self.select_stream(Select::Output);
        let flag = stream.write(buf.as_slice()).map(|count| regs.accum = count);
        regs.raise(flag);
    }
    fn get_descriptor(&self, regs: &mut Registers, select: Select) {
        regs.accum = self.select_descriptor(select).into();
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
