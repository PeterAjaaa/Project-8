#[derive(Debug)]
pub enum Instruction {
    FWD(usize),
    BWD(usize),
    INC(usize),
    DEC(usize),
    OUT(usize),
    IN(usize),
    STARTLOOPJUMP(usize),
    ENDLOOPJUMP(usize),
}
