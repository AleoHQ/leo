/*
namespace: Compile
expectation: Pass
input_file: input/main_tuple.in
*/

function main(x: (u8, bool, u8)) -> bool {
    return x.0 == 10 && x.1 == true && x.2 == 10;
}
