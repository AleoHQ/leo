/*
namespace: Compile
expectation: Pass
inputs:
 - i8_ne.in: |
    [main]
    a: i8 = 2;
    b: i8 = 5;
    c: bool = true;

    [registers]
    r0: bool = false;
 - i8_e.in: |
    [main]
    a: i8 = 5;
    b: i8 = 5;
    c: bool = false;

    [registers]
    r0: bool = false;
*/

function main(a: i8, b: i8, c: bool) -> bool{
    return (a != b) == c;
}
