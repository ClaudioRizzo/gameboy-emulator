struct Register {
    A: i8,
    F: i8,
    AF: i16,
    B: i8,
    C: i8,
    BC: i16,
    D: i8,
    E: i8,
    DE: i16,
    H: i8,
    L: i8,
    HL: i16,
    SP: i16,
    PC: i16,    
}

// Let's read 8bit opcodes, if we find a CB, then
// fetch should be caled twice
fn fetch(address: i16) -> i8 {
    0
}


