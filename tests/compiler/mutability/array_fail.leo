/*
namespace: Compile
expectation: Fail
*/

function main() {
    const a = [1u32];
    a[0] = 0;
}
