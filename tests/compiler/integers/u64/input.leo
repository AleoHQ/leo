/*
namespace: Compile
expectation: Pass
inputs:
 - u64_g.in: |
    [main]
    a: u64 = 2;
    b: u64 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u64, b: u64) -> bool {
    return a == b;
}
