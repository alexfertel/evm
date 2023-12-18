use common::cmd;
use common::get_binary_path;

mod common;

#[test]
fn assembler_reversability() {
    let expected = r"PUSH1 0x04
DUP1
PUSH1 0x00
JUMPDEST
DUP2
PUSH1 0x18
JUMPI
PUSH1 0x00
MSTORE8
PUSH1 0x01
PUSH1 0x00
RETURN
JUMPDEST
DUP3
ADD
SWAP1
PUSH1 0x01
SWAP1
SUB
SWAP1
PUSH1 0x05
JUMP";
    let binary_path = get_binary_path();
    let output = cmd(&binary_path, "assemble", &["-"], Some(expected));
    println!("{output:?}");
    let bytecode = String::from_utf8(output.stdout).unwrap();
    let output = cmd(&binary_path, "disassemble", &[bytecode.trim()], None);
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected.trim(), actual.trim());
}

#[test]
fn disassembler_reversability() {
    //             # stack
    // PUSH1, 4,   # n=4
    // DUP1,       # n=4, loops=4
    // PUSH1, 0,   # n=4, loops=4, result=0
    //
    // # loop_cond
    // # if loops != 0, jump to loop_body
    // JUMPDEST,
    // DUP2,       # n, loops, result, loops
    // PUSH1, 18,  # n, loops, result, loops, loop_body
    // JUMPI,      # n, loops, result
    //
    // # return result
    // PUSH1, 0,   # n, loops, result, m_result
    // MSTORE8,    # n, loops
    // PUSH1, 1,   # n, loops, mem_length
    // PUSH1, 0,   # n, loops, mem_length, mem_offset
    // RETURN,
    //
    // # loop_body
    // JUMPDEST,
    //
    // # result += n
    // DUP3,       # n, loops, result, n
    // ADD,        # n, loops, result'=n+result
    //
    // # loops -= 1
    // SWAP1,      # n, result', loops
    // PUSH1, 1,   # n, result', loops, 1
    // SWAP1,      # n, result', 1, loops
    // SUB,        # n, result', loops'=loops-1
    //
    // # restore stack
    // SWAP1,      # n, loops', result'
    //
    // # jump to loop_cond
    // PUSH1, 5,   # n, loops', result', loop_cond
    // JUMP,       # -> back to loop_cond
    let expected = "60048060005b8160125760005360016000f35b8201906001900390600556";
    let binary_path = get_binary_path();
    let output = cmd(&binary_path, "disassemble", &[expected], None);
    let asm = String::from_utf8(output.stdout).unwrap();
    let output = cmd(&binary_path, "assemble", &["-"], Some(&asm));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected.trim(), actual.trim());
}
