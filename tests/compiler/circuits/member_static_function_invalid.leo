/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    function echo(x: u32) -> u32 {
        return x;
    }
}

function main() {
    const err = Foo.echo(1u32); // Invalid, echo is a static function and must be accessed using `::`
}
