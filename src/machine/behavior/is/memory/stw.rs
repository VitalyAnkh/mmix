use machine::state::State;
use machine::state::mem::WydeAt;

/// store wyde
pub fn stw(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let a = op1.wrapping_add(op2);

    // Load x
    let res: i64 = state.gpr[x].into();

    // Store in memory
    state.mem[WydeAt(a)] = (res as i16).into();
}
