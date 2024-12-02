fn main() {
    // ranges are written infrom of each variable

    //  :: 8 BITS ::
    // --- Signed ---
    let _neg_signed8: i8 = -128;
    let _signed8: i8 = 127;
    // --- Unsigned  ---
    let _unsigned8: u8 = 255;

    // :: 16 BITS ::

    // Signed
    let _neg_signed16: i16 = -32768;
    let _signed16: i16 = 32767;
    // Unsigned
    let mut _unsigned16: u16 = 65535;
    _unsigned16 = 23;
    println!("Value of num : {}", _unsigned16);

    const CONST_INT: i128 = -92342131212312300009000554321233123123;
    println!("This is a constant varibale:  {}", CONST_INT);
}
