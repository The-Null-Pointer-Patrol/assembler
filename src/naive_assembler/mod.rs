use std::collections::HashSet;

use wg_2024::packet::{Fragment, FRAGMENT_DSIZE};

use crate::Assembler;

pub struct NaiveAssembler {}

impl Assembler for NaiveAssembler {
    fn reassemble(fragments: &[Fragment]) -> Vec<u8> {
        // Make sure that fragments are unique
        let mut existing = HashSet::new();
        for fragment in fragments {
            assert!(
                existing.insert(fragment.fragment_index),
                "Fragments cannot be reassemble due to duplicate fragment(s)"
            );
        }

        // Sort fragments according to indexes for cleaner reassembly
        let mut fragments = fragments.to_vec();
        fragments.sort_by_key(|f| f.fragment_index);
        let fragments = fragments;

        // Calculate the size needed for byte vector
        let size_for_alloc = if fragments.len() == 1 {
            /*
            In the case there is only one fragment, then the size
            shall be equal to the size of the fragment data.
            Panic if no first element exists.
             */
            fragments.first().unwrap().length as usize
        } else {
            /*
             In the case there are multiple fragments, then the size
             shall be equal to amount of fragments - 1 multiplied by
            the fragment data length constant plus the size of the
             last fragment data.
             */
            let size_of_last_fragment = fragments.last().unwrap().length as usize;
            let size_of_full_fragments = (fragments.len() - 1) * FRAGMENT_DSIZE;
            size_of_last_fragment + size_of_full_fragments
        };

        // Cheap initialization to avoid reallocation
        let mut byte_vector = vec![0; size_for_alloc];

        // Populate byte vector with the fragments
        for (index, fragment) in fragments.iter().enumerate() {
            let offset = index * FRAGMENT_DSIZE;
            if fragment.length == u8::try_from(FRAGMENT_DSIZE).unwrap() {
                byte_vector.splice(offset.., fragment.data);
            } else {
                byte_vector.splice(
                    offset..,
                    fragment.data[..fragment.length as usize].iter().copied(),
                );
            }
        }
        byte_vector
    }

    /// # Panics
    ///
    /// Will panic if `fragments` is empty or contains duplicate fragment indexes.
    fn disassemble(byte_vector: &[u8]) -> Vec<Fragment> {
        byte_vector
            .chunks(FRAGMENT_DSIZE)
            .enumerate()
            .map(|(index, chunk)| {
                // not the most efficient way to deal with this but it works
                let mut byte_array = [0; FRAGMENT_DSIZE];
                byte_array[..chunk.len()].copy_from_slice(chunk);

                Fragment {
                    fragment_index: u64::try_from(index).unwrap(),
                    total_n_fragments: u64::try_from(byte_vector.len().div_ceil(FRAGMENT_DSIZE))
                        .unwrap(),
                    length: u8::try_from(chunk.len()).unwrap(),
                    data: byte_array,
                }
            })
            .collect()
    }
}
