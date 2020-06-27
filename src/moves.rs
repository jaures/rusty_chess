pub mod util;

#[inline]
fn remove_edge(msk: u64) -> u64 {
    return 0x007e7e7e7e7e7e00 & msk;
}


pub fn rook_moves() -> [(u64,u64,u64,u64);64] {
    let top: u64 = 0x0101010101010100;
    let bottom: u64 = 0x0080808080808080;
    let left: u64 = 0xfe;
    let right: u64 = 0x7f00000000000000;

    let mut res: [(u64,u64,u64,u64);64] = [(0,0,0,0);64];

    for pos in 0..64_u8 {
        res[pos as usize] = (
            top << pos,
            bottom >> (63 - (pos )),
            (left << pos) & (0xff << (pos & 0xfa)),
            (right >> (63 - pos)) & (0xff << (pos & 0xf8)),
        );
    }

    return res;
}

pub fn bishop_moves() -> [(u64,u64,u64,u64);64] {
    const VERT_SPREAD: u64 = 0x0101010101010101;

    let NW_DIAG: u64 = 0x8040201008040200;
    let SE_DIAG: u64 = 0x40201008040201;
    let NE_SW_DIAG: u64 = 0x102040810204080;
    
    let mut res: [(u64,u64,u64,u64);64] = [(0,0,0,0);64];

    for pos in 0..64_u8 {
        res[pos as usize] = (
            (NW_DIAG << pos) & ( (0xff - ((1 << (pos & 0x7)) - 1)) * VERT_SPREAD),
            (SE_DIAG >> (63 - pos)) & ( ((1 << (pos & 0x7)) - 1) * VERT_SPREAD),
            (NE_SW_DIAG << pos) & ((1 << (pos & 0x7)) - 1) * VERT_SPREAD,
            (NE_SW_DIAG >> (63 - pos)) & ((0xff - ((1 << (pos & 0x7)) - 1)) * VERT_SPREAD),
        );
    }

    return res;
}

pub fn knight_moves() -> [u64;64] {
    const KN_ATK: u64 = 0xa1100110a;
    const KN_MSK: u64 = 0x3f3f3f3f3f3f3f3f;

    let mut res: [u64;64] = [0;64];

    // Get For indicies 0 - 18
    //(KN_MSK >> (18 - pos)) & 
    for pos in 0..19_usize {
        res[pos] = (KN_ATK >> (18 - pos)) & (KN_MSK << ((pos & 0x4) >> 1));
    }

    // Get For indicies 19 - 63
    for pos in 19..64_usize {
        res[pos] = (KN_ATK << (pos - 18)) & (KN_MSK << ((pos & 0x4) >> 1));
    }

    return res;
}

pub fn pawn_moves() -> [(u64, u64);64] {
    const PN_ATK:u64 =  0x280;
    const PN_MSK:u64 =  0x1f1f1f1f1f1f1f1f;
    
    let mut res: [(u64,u64);64] = [(0,0);64];
    
    for pos in 8..56_usize {
        res[pos] = (
            (1 << (pos + 8)) | ((1 >> pos) << (8 + pos)),
            (PN_ATK << pos) & (PN_MSK << (pos & 0x6))
        );
    }

    return res;
}