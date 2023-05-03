use crate::reg::Registers;
use util::Stream;

const STDIN: u8 = 0;
const STDOUT: u8 = 1;
const STDERR: u8 = 2;
const STREAM_COUNT: usize = 1 << u8::BITS;

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
        *this.get_mut(STDIN) = Stream::Stdin;
        *this.get_mut(STDOUT) = Stream::Stdout;
        *this.get_mut(STDERR) = Stream::Stderr;
        this
    }
    fn get_mut(&mut self, descriptor: u8) -> &mut Stream {
        &mut self.array[usize::from(descriptor)]
    }
}

pub struct Maps {
    array: StreamArray,
    input: u8,
    output: u8,
}
impl Maps {
    pub fn new() -> Self {
        Self {
            array: StreamArray::new(),
            input: STDIN,
            output: STDOUT,
        }
    }
    pub fn init(&mut self, input: Stream, output: Stream) {
        *self.array.get_mut(STDIN) = input;
        *self.array.get_mut(STDOUT) = output;
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
            0 => regs.get_descriptor(|| self.input),
            1 => regs.get_descriptor(|| self.output),
            2 => regs.set_descriptor(|desc| self.input = desc),
            3 => regs.set_descriptor(|desc| self.output = desc),
            _ => unimplemented!(),
        }
    }
}
