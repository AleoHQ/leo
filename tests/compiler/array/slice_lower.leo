/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    const arr: [u32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    const expected: [u32; 2] = [0, 1];

    const actual = arr[..2];     // Should produce [0, 1]

    return (expected == actual) == y;
}
