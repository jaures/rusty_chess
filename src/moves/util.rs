/**
 * Flip a bitboard vertically about the centre ranks.
 * Rank 1 is mapped to rank 8 and vice versa.
 * @param val any bitboard
 * @return bitboard val flipped vertically
 */
 pub fn flip_vertical(val: u64) -> u64 {
   const K1: u64 = 0x00FF00FF00FF00FF;
   const K2: u64 = 0x0000FFFF0000FFFF;
   let val = ((val >>  8) & K1) | ((val & K1) <<  8);
   let val = ((val >> 16) & K2) | ((val & K2) << 16);
   let val = ( val >> 32)       | ( val       << 32);
   return val;
}

/**
 * Mirror a bitboard horizontally about the center files.
 * File a is mapped to file h and vice versa.
 * @param val any bitboard
 * @return bitboard val flipped horizontally
 */
 pub fn flip_horizontal (val: u64) -> u64 {
   const K1: u64 = 0x5555555555555555;
   const K2: u64 = 0x3333333333333333;
   const K4: u64 = 0x0f0f0f0f0f0f0f0f;

   let val = ((val >> 1) & K1) | ((val & K1) << 1);
   let val = ((val >> 2) & K2) | ((val & K2) << 2);
   let val = ((val >> 4) & K4) | ((val & K4) << 4);
   
   return val;
}


/**
* Flip a bitboard about the diagonal a1-h8.
* Square a1 is mapped to h8 and vice versa.
* @param val any bitboard
* @return bitboard val flipped about diagonal a1-h8
*/
pub fn flip_diag_a1h8(val: u64) -> u64 {
   const K1: u64 = 0xaa00aa00aa00aa00;
   const K2: u64 = 0xcccc0000cccc0000;
   const K4: u64 = 0xf0f0f0f00f0f0f0f;
   
   let t    = val ^ (val << 36) ;
   let val  = val ^ (K4 & (t ^ (val >> 36)));
   
   let t    = K2 & (val ^ (val << 18));
   let val  = val ^ (t ^ (t >> 18));
   
   let t    = K1 & (val ^ (val <<  9));
   let val  = val ^ (t ^ (t >>  9));
   
   return val;
}

/**
 * Flip a bitboard about the anti-diagonal a8-h1.
 * Square h1 is mapped to a8 and vice versa.
 * @param val any bitboard
 * @return bitboard val flipped about anti-diagonal a8-h1
 */
 pub fn flip_diag_a8h1(val: u64) -> u64 {
   const K1: u64 = 0x5500550055005500;
   const K2: u64 = 0x3333000033330000;
   const K4: u64 = 0x0f0f0f0f00000000;

   let t    = K4 & (val ^ (val << 28));
   let val  = val ^ (t ^ (t >> 28));

   let t    = K2 & (val ^ (val << 14));
   let val  = val ^ (t ^ (t >> 14));

   let t    = K1 & (val ^ (val <<  7));
   let val  = val ^ (t ^ (t >>  7));

   return val;
}


/**
* Rotate a bitboard by 180 degrees.
* Square a1 is mapped to h8, and a8 is mapped to h1.
* @param val any bitboard
* @return bitboard val rotated 180 degrees
*/
pub fn rotate180 (val: u64) -> u64 {
   const H1: u64 = 0x5555555555555555;
   const H2: u64 = 0x3333333333333333;
   const H4: u64 = 0x0F0F0F0F0F0F0F0F;
   const V1: u64 = 0x00FF00FF00FF00FF;
   const V2: u64 = 0x0000FFFF0000FFFF;

   let val = ((val >>  1) & H1) | ((val & H1) <<  1);
   let val = ((val >>  2) & H2) | ((val & H2) <<  2);
   let val = ((val >>  4) & H4) | ((val & H4) <<  4);
   let val = ((val >>  8) & V1) | ((val & V1) <<  8);
   let val = ((val >> 16) & V2) | ((val & V2) << 16);
   let val = ( val >> 32)       | ( val       << 32);
   
   return val;
}

/**
 * Rotate a bitboard by 90 degrees clockwise.
 * @param val any bitboard
 * @return bitboard val rotated 90 degrees clockwise
 */
 pub fn rotate90_clockwise (val: u64) -> u64 {
   return flip_vertical (flip_diag_a1h8 (val) );
}

/**
 * Rotate a bitboard by 90 degrees anticlockwise.
 * @param val any bitboard
 * @return bitboard val rotated 90 degrees anticlockwise
 */
 pub fn rotate90_anti_clockwise (val: u64) -> u64 {
   return flip_vertical (flip_diag_a8h1 (val) );
}