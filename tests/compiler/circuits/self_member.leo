/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    f: u32;

    function bar(self) -> u32 {
        return self.f;
    }
}

function main(y: bool) -> bool {
    const a = Foo { f: 1u32 };
    const b = a.bar();

    return (b == 1u32) == y;
}
