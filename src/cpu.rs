use std::collections::HashMap;

struct Register {
    a: i8,
    f: i8,
    b: i8,
    c: i8,
    d: i8,
    e: i8,
    h: i8,
    l: i8,
    sp: i16,
    pc: i16,    
}

struct Opcodes {
    ldrr_bb: fn(i16)->(i8),
}

struct Clock {
    machine_cycle: i8,
    time_cycle: i8,
}




// Let's read 8bit opcodes, if we find a CB, then
// fetch should be caled twice
pub fn fetch(address: i16) -> i8 {
    let opcs = Opcodes {
        ldrr_bb: |address| address as i8, 
    };
    
    (opcs.ldrr_bb)(address)
}








