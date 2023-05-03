use crate::reg::Registers;
use util::Stream;

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
        let array = [(); STREAM_COUNT].map(|_| Stream::Null);
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

pub struct Maps {
    array: StreamArray,
    input: Descriptor,
    output: Descriptor,
}
impl Maps {
    pub fn new() -> Self {
        Self {
            array: StreamArray::new(),
            input: Descriptor::STDIN,
            output: Descriptor::STDOUT,
        }
    }
    pub fn init(&mut self, input: Stream, output: Stream) {
        *self.array.get_mut(Descriptor::STDIN) = input;
        *self.array.get_mut(Descriptor::STDOUT) = output;
    }
    pub fn getchar(&mut self, regs: &mut Registers) {
        let stream = self.array.get_mut(self.input);
        regs.getchar(|| stream.getchar());
    }
    pub fn putchar(&mut self, regs: &mut Registers) {
        let stream = self.array.get_mut(self.output);
        regs.putchar(|data| stream.putchar(data));
    }
    pub fn stream(&mut self, regs: &mut Registers) {
        match regs.data {
            0 => regs.get_descriptor(|| self.input.into()),
            1 => regs.get_descriptor(|| self.output.into()),
            2 => regs.set_descriptor(|desc| self.input = Descriptor::new(desc)),
            3 => regs.set_descriptor(|desc| self.output = Descriptor::new(desc)),
            _ => unimplemented!(),
        }
    }
}
