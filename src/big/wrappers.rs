#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, int64_t};

// TODO: autogenerate this part!
pub const NLEN:usize = 5;      // use amcl_build command to get this
pub type chunk = int64_t;  // use amcl_build command to get this
pub const MODBYTES:usize = 32; // use amcl_build command to get this
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

pub type BIG = [ chunk; NLEN ];

#[macro_export]
macro_rules! BIG_ZERO {
    () => {
        [ 0; NLEN ];
    };
}

extern {
    // TODO: maybe move to separate module "rom"
    static MConst: chunk;
    static Modulus: BIG;
    static CURVE_Order: BIG;
    static CURVE_Cof: BIG;
    static CURVE_B: BIG;
    static CURVE_Bnx: BIG;
    static CURVE_Cru: BIG;
    static CURVE_Fra: BIG;
    static CURVE_Frb: BIG;
    static CURVE_Pxa: BIG;
    static CURVE_Pxb: BIG;
    static CURVE_Pya: BIG;
    static CURVE_Pyb: BIG;
    static CURVE_Gx: BIG;
    static CURVE_Gy: BIG;
    static CURVE_W: [BIG; 2];
    static CURVE_SB: [[BIG; 2]; 2];
    static CURVE_WB: [BIG; 4];
    static CURVE_BB: [[BIG; 4]; 4];
    // ^^^^^^^
    
    pub fn BIG_nbits(a: *const BIG) -> c_int;
    pub fn BIG_copy(d: *mut BIG, s: *const BIG) -> c_void;
    pub fn BIG_shr(a: *mut BIG, k: c_int) -> c_void;
    pub fn BIG_rcopy(b: *mut BIG, a: *const BIG) -> c_void;
}

pub fn big_to_hex(a: &BIG) -> String {
    let mut ret: String = String::with_capacity(MODBYTES*2);
    let mut b: BIG = BIG_ZERO!();
    let mut len: usize;

    unsafe {
        len = BIG_nbits(a) as usize;
    }

    if len % 4 == 0 {
        len /= 4;
    } else {
        len /= 4;
        len += 1;
    }

    if len < MODBYTES * 2 {
        len=MODBYTES*2;
    }

    for i in (0..len).rev() {
        unsafe {
            BIG_copy(&mut b, a);
            BIG_shr(&mut b, (i*4) as i32);
        }
        ret.push_str(&format!("{:X}", b[0]&15));
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let a: BIG = BIG_ZERO!();
        assert_eq!(big_to_hex(&a), "0000000000000000000000000000000000000000000000000000000000000000");
    }
}
