/*
namespace: Compile
expectation: Pass
inputs:
 - i64_l.in: |
    [main]
    a: i64 = 2;
    b: i64 = 4;
    c: bool = true;

    [registers]
    r0: bool = false;
 - i64_e.in: |
    [main]
    a: i64 = 2;
    b: i64 = 2;
    c: bool = false;

    [registers]
    r0: bool = false;
*/

function main(a: i64, b: i64, c: bool) -> bool{
    return a < b == c;
}
