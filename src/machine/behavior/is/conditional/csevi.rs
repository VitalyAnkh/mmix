use machine::state::State;

/// conditional set if even immediate
pub fn csevi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    if op1 % 2 == 0 {
        state.gpr[x] = (z as u64).into();
    }
}
