mod instructions;

use std::fs::read;
use std::path::Path;

/// https://medium.com/swlh/an-introduction-to-jvm-bytecode-5ef3165fae70
/// https://en.wikipedia.org/wiki/List_of_Java_bytecode_instructions
/// https://blogs.oracle.com/javamagazine/post/understanding-java-method-invocation-with-invokedynamic
/// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html


fn main() {
    println!("{}", read_class_file(Path::new("./data/Example.class")));
}

use u8 as w1;
use u16 as w2;
use u32 as w4;

fn read_class_file(path: &Path) -> String {
    let bytes: Vec<w1> = read(path).unwrap();
    // bytes.

    let magic: u32 = read_u4(&bytes[0..4]);
    let minor: u16 = read_u2(&bytes[4..6]);
    let major: u16 = read_u2(&bytes[6..8]);
    format!(
        "magic = {magic:#X};\n\
        minor = {minor}\n\
        major = {major}\
        ",
        magic=magic,
        minor=minor,
        major=major
    )
}



macro_rules! impl_read_n {
    ($fn_name:ident, $t:ty, $width:expr) => {
        fn $fn_name(bytes: &[w1]) -> $t {
            let mut u: $t = 0;
            for i in (0..$width).rev() {
                u |= ((bytes[i] as $t) << 8*(($width-1-i)))
            }
            return u;
        }
    };
}

// impl_read_n!(read_u1, w1, 1);
impl_read_n!(read_u2, w2, 2);
impl_read_n!(read_u4, w4, 4);
//
// fn read_n<Un: Num, const N: usize>(bytes: &[u8]) -> Un {
//     let mut u: Un = Num::zero();
//     for i in (0..N).rev() {
//         u |= ((bytes[i] as Un) << (N-i))
//     }
//     return u;
// }

// fn read_u4(bytes: &[u8; 4]) -> u32 {
//     return ((bytes[0] as u32) << 24)
//         | ((bytes[1] as u32) << 16)
//         | ((bytes[2] as u32) << 8)
//         | ((bytes[3] as u32) << 0)
// }
//
// fn read_u2(bytes: &[u8; 2]) -> u16 {
//     return ((bytes[0] as u16) << 8)
//         | ((bytes[1] as u16) << 0)
// }
