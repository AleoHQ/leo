function main(a: u32, b: u32) {
    let c = 0u32;

    if a == 1 {
        c = 1;
    } else if a == 2 {
        c = 2;
    } else {
        c = 3;
    }

    console.assert(c == b);
}