circuit Foo {
    a: u32;
}

circuit Bar {
    function bar() {
        const f = Foo { a: 0u32 };
    }
}

function main() {
    const b = Bar::bar();
}