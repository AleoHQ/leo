/*
namespace: Compile
expectation: Fail
*/

function main() {
    const b: [[[u8; 2]; 3]; 4] = [[[0; 4]; 3]; 2]; // initializer (incorrectly reversed ordering)
}
