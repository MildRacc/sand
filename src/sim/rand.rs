

static mut XOR_RAND: u32 = 6964;

pub fn rand() -> u32
{
    unsafe
    {
        XOR_RAND ^= XOR_RAND << 13;
        XOR_RAND ^= XOR_RAND >> 17;
        XOR_RAND ^= XOR_RAND << 5;
        XOR_RAND
    }
}
