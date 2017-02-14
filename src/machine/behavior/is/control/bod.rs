use machine::state::State;

pub fn bod(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: i64 = state.gpr[x].into(); 
    let ra: u64 = ((y as u64) << 8) + (z as u64);
    let at: u64 = state.pc.into();

    // Execute
    let res: u64 = at + 4 * ra;

    // Store result
    if op1 % 2 == 1 {
    	state.pc = res.into();
    }
}
