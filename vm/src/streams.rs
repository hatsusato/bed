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
            5 => self.argv(regs),
            _ => unimplemented!(),
        }
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
            .and_then(|arg| self.write_stream(arg.as_bytes()))
            .map(|_| ());
        regs.raise(flag);
    }
    fn read_number(&mut self, count: u8) -> Option<usize> {
        const SIZE: usize = std::mem::size_of::<usize>();
        if usize::from(count) < SIZE {
            let mut bytes = [0; SIZE];
            self.read_stream(&mut bytes[..usize::from(count)])
                .and_then(|len| bool_to_option(len == count))
                .and(Some(bytes))
                .map(usize::from_le_bytes)
        } else {
            None
        }
    }
    fn write_number(&mut self, number: usize) -> Option<u8> {
        let bytes = number.to_le_bytes();
        let count = bytes.into_iter().rev().skip_while(|&x| x == 0).count();
        self.write_stream(&bytes[..count])
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
    fn read_stream(&mut self, buf: &mut [u8]) -> Option<u8> {
        let stream = self.select_stream(Select::Input);
        stream.read(buf)
    }
    fn write_stream(&mut self, buf: &[u8]) -> Option<u8> {
        let stream = self.select_stream(Select::Output);
        stream.write(buf)
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

fn bool_to_option(x: bool) -> Option<()> {
    if x {
        Some(())
    } else {
        None
    }
}
