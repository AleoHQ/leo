/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function array_3x2_nested() -> [[u8; 2]; 3] {
    return [[0u8; 2]; 3];
}

function array_3x2_tuple() -> [[u8; 2]; 3] {
    return [0u8; (3, 2)];
}

function main(y: bool) -> bool {
    const a = array_3x2_nested();
    const b = array_3x2_tuple();
    return y == true;
}
