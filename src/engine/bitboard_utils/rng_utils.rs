//generate random 32 bit numbers using XORShift algorithm
pub fn generate_random_32bit(mut seed:u32) -> u32 {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    seed
}

//generate random 64 bit numbers with low number of 1s
pub fn generate_random_64bit(mut seed:u32) -> (u64,u32) {
    seed = generate_random_32bit(seed);
    let n1 = (seed as u64) & 0xFFFF;
    seed = generate_random_32bit(seed);
    let n2 = (seed as u64) & 0xFFFF;
    seed = generate_random_32bit(seed);
    let n3 = (seed as u64) & 0xFFFF;
    seed = generate_random_32bit(seed);
    let n4 = (seed as u64) & 0xFFFF;
    let num = (n1) | (n2<<16) | (n3<<32) | (n4<<48);
    (num,seed)
}

//generate candidate for magic number
pub fn generate_magic_number_candidate(mut seed:u32) -> (u64,u32) {
    let n1;
    (n1,seed) = generate_random_64bit(seed);
    let n2;
    (n2,seed) = generate_random_64bit(seed);
    let n3;
    (n3,seed) = generate_random_64bit(seed);
    let candidate = n1 & n2 & n3;
    (candidate,seed)
}