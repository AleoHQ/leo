function one() -> u32 {
    return 1u32;
}

function main() {
    let a = 0u32;

    for i in 0..10 {
        a += one();
    }

    console.assert(a == 10u32);
}