/*
namespace: Compile
expectation: Fail
*/

// Let variables are immutable by default.
function main() {
    const a = 1u32;
    a = 0;
}