#[derive(Debug)]
pub struct State {
    pc:  u16,
    sp:  u8,
    p:   u8,
    acc: u8,
    x:   u8,
    y:   u8,
}


//
pub fn cpu_init(mem: &[i8; 65536]) -> State {
    let shit: State = State {
        pc:  0,
        sp:  0xfd,
        acc: 0,
        x:   0,
        y:   0,
        p:   34
    };
    shit
}

pub fn cpu_reset(state: &mut State) {
    state.sp -= 3;

}
