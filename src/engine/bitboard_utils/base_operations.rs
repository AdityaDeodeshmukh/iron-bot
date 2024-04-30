//get the bit value from a bitboard at a particular square
macro_rules! get_bit {
    ($bitboard:expr,$square:expr) => {
        if ($bitboard) & (1 << ($square)) > 0 {1} else {0}
    };
}
pub(crate) use get_bit;


//set the bit value to 1 on a bitboard at a particular square
macro_rules! set_bit {
    ($bitboard:expr,$square:expr) => {
        ($bitboard) | (1 << ($square))
    };
}
pub(crate) use set_bit;


//set the bit value to 0 on a bitboard at a particular square
macro_rules! pop_bit {
    ($bitboard:expr,$square:expr) => {
        (($bitboard)&!(1 << ($square)))
    };
}
pub(crate) use pop_bit;

//get the number of bits that are set in a bitboard (Brian Kernighan's Algorithm)
#[inline]
pub fn _get_bit_nums(mut bitboard: u64) -> u8 {
    let mut ans = 0;
    while bitboard>0 {
        ans+=1;
        bitboard = bitboard & (bitboard-1);
    }
    ans
}

//get the number of bits that are set in a bitboard (BMI)
#[inline(never)]
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
pub unsafe fn get_bit_nums(a: u64) -> u32 {
    a.count_ones()
}

//get the number of trailing 0 bits that are set in a bitboard (BMI)
#[inline(never)]
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "bmi1"))]
pub unsafe fn get_lsb_index(a: u64) -> u8 {
    a.trailing_zeros() as u8
}


//display a bitboard in a chess board format
pub fn print_bit_board(bitboard: u64) {
    for row in (0..=7).rev() {
        print!("{}   ",row+1);
        for file in 0..=7 {
            let square = row*8 + file;
            
            print!("{} ",if get_bit!(bitboard,square) == 0 {0} else {1});
        }
        println!("");
    }
    println!("");
    println!("    a b c d e f g h");
    println!("Num:{}",bitboard);
}