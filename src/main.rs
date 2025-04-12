//------------------------------------------------------------------------------
// Rust DFS with Memoization for 3x3 Board State Exploration
//
// Approach:
// This program represents a 3x3 board as a 64-bit integer where each cell is encoded using 4 bits.
// It uses a Depth-First Search (DFS) algorithm with memoization to explore all possible board states
// up to a given depth (or until the board is full). The DFS is implemented recursively, and it caches 
// intermediate results in a HashMap to avoid redundant recalculations of the same state.
// 
// Algorithm:
// 1. Encode the board in a 64-bit integer, each cell using 4 bits.
// 2. Use helper functions `get_cell` and `set_cell` to read and modify the state.
// 3. Implement DFS: For each state, if the maximum depth is reached or the board is full, compute a hash 
//    of the board state and add it to the total; otherwise, for every empty cell, generate new states 
//    based on the available moves. If a cell has at least two non-empty neighboring cells, apply valid 
//    merge combinations ("combos") to generate new states.
// 4. Use memoization (hashing the state and current turn) to cache and quickly return results for states 
//    that have already been computed.
// 
// Time Complexity:
// Worst-case time complexity is exponential without memoization, but the memoization greatly prunes
// redundant computations. In practice, the effective complexity depends on the number of unique states 
// encountered.
// 
// Space Complexity:
// The algorithm uses O(m) space for memoization, where m is the number of unique states stored. The
// recursion depth is O(max_depth), which is generally much smaller.
//
//------------------------------------------------------------------------------

// Import necessary standard library modules.
use std::io::BufRead;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

//
//=== FxHasher optimized for u64 ===//
//

// Custom hasher that optimizes for u64 keys.
#[derive(Default)]
struct FxHasher(u64);

// Implement the Hasher trait for FxHasher.
impl Hasher for FxHasher {
    #[inline]
    fn write_u64(&mut self, i: u64) {
        // Update the internal hash state using wrapping arithmetic and a constant multiplier.
        self.0 = self.0.wrapping_add(i)
                  .wrapping_mul(0x517cc1b727220a95);
        // Finalize the hash value by XORing with its shifted version.
        self.0 ^= self.0 >> 32;
    }

    #[inline]
    fn write(&mut self, _: &[u8]) {
        // This function should never be called because only u64 keys are used.
        unreachable!("Only used with u64 keys")
    }

    #[inline]
    fn finish(&self) -> u64 {
        // Return the current hash value.
        self.0
    }
}

// BuildHasher for FxHasher allowing it to be used in HashMap.
#[derive(Clone, Default)]
struct FxBuildHasher;

impl BuildHasher for FxBuildHasher {
    type Hasher = FxHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        // Initialize FxHasher with a seed constant.
        FxHasher(0x9E3779B97F4A7C15)
    }
}

//
//=== Constants and Definitions for the 3x3 Board ===//
//

const SIZE: usize = 3;
const MODULO: u64 = 1 << 30;
const MODULO_MASK: u64 = MODULO - 1;
// Reserve 6 bits for the turn value in the memoization key.
const TURN_BITS: u32 = 6;

// Define the neighbors for each cell on the 3x3 board.
const NEIGHBORS: [&[usize]; 9] = [
    &[1, 3],
    &[0, 2, 4],
    &[1, 5],
    &[0, 4, 6],
    &[1, 3, 5, 7],
    &[2, 4, 8],
    &[3, 7],
    &[4, 6, 8],
    &[5, 7],
];

// Type alias for a slice of neighbor indices.
type Combo = &'static [usize];

// Predefined merge combinations for different neighbor counts.
const COMBOS: [&[Combo]; 5] = [
    &[], // 0 neighbor
    &[], // 1 neighbor
    &[&[0, 1]], // 2 neighbors
    &[
        &[0, 1],
        &[0, 2],
        &[1, 2],
        &[0, 1, 2],
    ], // 3 neighbors
    &[
        &[0, 1], &[0, 2], &[0, 3],
        &[1, 2], &[1, 3], &[2, 3],
        &[0, 1, 2], &[0, 1, 3], &[0, 2, 3], &[1, 2, 3],
        &[0, 1, 2, 3],
    ],
];

//
//=== State Manipulation Functions (each cell uses 4 bits) ===//
//

#[inline(always)]
fn get_cell(state: u64, idx: usize) -> u64 {
    // Get the value of the cell at index `idx` by shifting and masking.
    (state >> (idx << 2)) & 0xF
}

#[inline(always)]
fn set_cell(state: u64, idx: usize, value: u64) -> u64 {
    // Calculate the bit shift for the cell based on its index.
    let shift = idx << 2;
    // Clear the bits corresponding to that cell and set the new value (only lower 4 bits are used).
    (state & !(0xF << shift)) | ((value & 0xF) << shift)
}

#[inline(always)]
fn is_full(state: u64) -> bool {
    // The board is considered full if every cell is non-zero.
    // This is determined by checking that each group of 4 bits has at least one bit set.
    let any_bit_set = state | (state >> 1) | (state >> 2) | (state >> 3);
    (any_bit_set & 0x111111111) == 0x111111111
}

#[inline(always)]
fn compute_hash(state: u64) -> u64 {
    // Compute a hash value for the board state.
    let mut s = state;
    let mut hash = 0u64;
    for _ in 0..9 {
        // Combine each cell value into the hash using a modulo-masked arithmetic.
        hash = (hash * 10 + (s & 0xF)) & MODULO_MASK;
        s >>= 4;
    }
    hash
}

//
//=== Optimized DFS with Memoization ===//
//

// Recursive DFS function that explores board states.
// Parameters:
// - state: current board state encoded in u64.
// - turn: current DFS depth or move number.
// - max_depth: maximum allowed depth for DFS.
// - memo: memoization table to cache results and avoid redundant computations.
// - total: accumulator for the computed hash values (modulo MODULO_MASK).
fn dfs(state: u64, turn: u64, max_depth: u64, memo: &mut HashMap<u64, u64, FxBuildHasher>, total: &mut u64) {
    // If maximum depth is reached or the board is full, add the state's hash to total.
    if turn == max_depth || is_full(state) {
        *total = (*total + compute_hash(state)) & MODULO_MASK;
        return;
    }

    // Create a unique key by combining state and turn.
    let key = (state << TURN_BITS) | turn;
    // Check if the result for this key is already computed.
    if let Some(&val) = memo.get(&key) {
        *total = (*total + val) & MODULO_MASK;
        return;
    }

    let start = *total;

    // Extract the board cells into a local array for repeated access.
    let mut cells = [0u64; 9];
    for i in 0..9 {
        cells[i] = get_cell(state, i);
    }

    // Build a bitmask for empty cells: each bit corresponds to a cell being empty.
    let mut empty_mask: u16 = 0;
    for idx in 0..9 {
        empty_mask |= ((cells[idx] == 0) as u16) << idx;
    }

    // Iterate over all empty cells.
    while empty_mask != 0 {
        // Obtain the index of the least significant empty cell.
        let idx = empty_mask.trailing_zeros() as usize;
        empty_mask &= empty_mask - 1;

        let neighbors = NEIGHBORS[idx];
        let mut valid_values = [0u64; 4];
        let mut valid_masks = [0u64; 4];
        let mut v_count = 0;
        // Collect values and positions of non-empty neighboring cells.
        // excluding those with value 6, since they can't be used in merges.
        for &pos in neighbors {
            if cells[pos] != 0 && cells[pos] != 6 {
                valid_masks[v_count] = 0xF << (pos << 2);
                valid_values[v_count] = cells[pos];
                v_count += 1;
            }
        }

        // If fewer than two neighbors are non-empty, simply set the empty cell to 1.
        if v_count < 2 {
            let new_state = set_cell(state, idx, 1);
            dfs(new_state, turn + 1, max_depth, memo, total);
            continue;
        }

        // Use predefined combos based on the number of non-empty neighbors.
        let combos = COMBOS[v_count];
        let idx_shift = idx << 2;
        let mut found = false;
        for &combo in combos {
            let mut sum = 0;
            for &i in combo {
                sum += valid_values[i];
                if sum > 6 { break; }
            }
            if sum > 6 { continue; }

            let mask = combo.iter().fold(0, |acc, &i| acc | valid_masks[i]);
            let new_state = (state & !mask) | (sum << idx_shift);
            dfs(new_state, turn + 1, max_depth, memo, total);
            found = true;
        }
        // If no combo was applicable, set the cell to the default value 1.
        if !found {
            let new_state = set_cell(state, idx, 1);
            dfs(new_state, turn + 1, max_depth, memo, total);
        }
    }

    // Calculate the incremental result for this DFS branch and cache it.
    let val = (*total + MODULO - start) & MODULO_MASK;
    memo.insert(key, val);
}

//
//=== Main Function ===//
//
// Reads input from standard input.
// The first line is the maximum depth (number of turns).
// The following SIZE lines represent the board rows.
// Then, the DFS is invoked to explore the states and compute the result,
// which is finally printed to the standard output.
fn main() -> std::io::Result<()> {
    // Lire depuis input.txt
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let depth: u64 = lines.next().unwrap()?.trim().parse().unwrap();

    let mut initial_state: u64 = 0;
    for i in 0..SIZE {
        let row: Vec<u64> = lines.next().unwrap()?
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..SIZE {
            initial_state = set_cell(initial_state, i * SIZE + j, row[j]);
        }
    }
    
    // Create a HashMap for memoization with a custom hasher and preallocated capacity.
    let mut memo: HashMap<u64, u64, FxBuildHasher> =
        HashMap::with_capacity_and_hasher(1 << 16 , FxBuildHasher::default());

    let mut total = 0;
    dfs(initial_state, 0, depth, &mut memo, &mut total);
    
    println!("{}", total);

    Ok(())
    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test_case(path: &str) {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        // Open the input file
        let file = File::open(path).expect("Failed to open file");
        let mut lines = BufReader::new(file).lines();

        // Read the depth from the first line
        let depth: u64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Read the 3x3 board
        let mut initial_state: u64 = 0;
        for i in 0..SIZE {
            let row: Vec<u64> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for j in 0..SIZE {
                initial_state = set_cell(initial_state, i * SIZE + j, row[j]);
            }
        }

        // Read the expected result
        let expected: u64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Run the DFS
        let mut memo: HashMap<u64, u64, FxBuildHasher> =
            HashMap::with_capacity_and_hasher(1 << 16, FxBuildHasher::default());
        let mut total = 0;
        dfs(initial_state, 0, depth, &mut memo, &mut total);

        // Print and compare results
        println!("[{}] Expected: {}", path, expected);
        println!("[{}] Got     : {}", path, total);
        assert_eq!(total, expected);
    }

    #[test]
    fn test_multiple_cases() {
        let test_files = vec![
            "tests/data/2_states.txt",
            "tests/data/6_states.txt",
            "tests/data/2_unique_states.txt",
            "tests/data/11_unique_states.txt",
            "tests/data/20_unique_states.txt",
            "tests/data/241_unique_states.txt",
            "tests/data/2168_unique_states.txt",
            "tests/data/4154_unique_states.txt",
            "tests/data/4956_unique_states.txt",
            "tests/data/6044_unique_states.txt",
            "tests/data/93190_unique_states.txt",
            "tests/data/94956_unique_states.txt",

            "tests/data/input1.txt",
            "tests/data/input2.txt",
            "tests/data/input3.txt",
            "tests/data/input4.txt",
            "tests/data/input5.txt",
            "tests/data/input6.txt",
            "tests/data/input7.txt",
            "tests/data/input8.txt",
            "tests/data/input9.txt",
            "tests/data/input10.txt",
            "tests/data/input11.txt",
            "tests/data/input12.txt",
            "tests/data/input13.txt",
            "tests/data/input14.txt",
            "tests/data/input15.txt",
            "tests/data/input16.txt",
            "tests/data/input17.txt",
            "tests/data/input18.txt",
            "tests/data/input19.txt",
            "tests/data/input20.txt",
        ];

        for file in test_files {
            run_test_case(file);
        }
    }
}