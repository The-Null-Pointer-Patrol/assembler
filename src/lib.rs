///! This library provides an Assembler trait with two functions `disassemble` and `reassemble`.
///! There is a simple struct implementing this trait; `NaiveAssembler`.
use wg_2024::packet::Fragment;

pub mod naive_assembler;

pub trait Assembler {
    /// Reassembles data from of fragments (`&[Fragment]`) into a single byte vector.
    ///
    /// # Parameters
    /// - `fragments`: Slice of fragments that are to be reassembled.
    ///
    /// # Returns
    /// Returns byte vector of the data reassembled from fragments.
    fn reassemble(fragments: &[Fragment]) -> Vec<u8>;

    /// Fragmentizes a byte slice into a vector of fragments.
    ///
    /// # Parameters
    /// - `byte_vector`: Bytes to be fragmentized.
    ///
    /// # Returns
    /// Returns a fragment vector.
    fn disassemble(byte_vector: &[u8]) -> Vec<Fragment>;
}
