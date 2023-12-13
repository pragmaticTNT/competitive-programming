use competition::*;

fn main() {
    run(solve).unwrap()
}

fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
    let n: usize = lines.next().unwrap().parse()?;
    // b'0' for black, b'1' for white, note not 0 and 1.
    let bitstring: Vec<u8> = lines.next().unwrap().into_bytes(); // TODO: Something better than a string
    let mut bitstring: Vec<bool> = bitstring.into_iter().map(|x| x == b'1').collect();
    let q: usize = lines.next().unwrap().parse()?;
    let bs: Vec<usize> = lines.take(q).map(|line| line.parse()).collect::<std::result::Result<_, _>>()?;

    let mut flips = vec![false; bitstring.len()];
    let mut ret: i64 = 0;
    for i in 0.. n {
        if bitstring[i] {
            flips[i] = true;
            ret += 1;
            for j in (i..n).step_by(i+1) {
                bitstring[j] ^= true;
            }
        }
    }

    // // Part (a)
    // for bi in bs {
    //     ret += if !flips[bi-1] {
    //         1
    //     } else {
    //         -1
    //     };
    //     flips[bi-1] ^= true;
    // }

    // Part (b)
    // println!("flips: {:?}", flips);
    ret += if !flips[bs[0]-1] { 1 } else { -1 };
    flips[bs[0]-1] ^= true;
    let mut counter = ret;

    for bi in bs.into_iter().skip(1) {
        counter += if !flips[bi-1] { 1 } else { -1 };
        // println!("bi: {}, counter: {}", bi, counter);
        ret += counter;
        flips[bi-1] ^= true;
    }

    Ok(ret as usize)
}

// fn solve(lines: &mut dyn Iterator<Item=String>) -> Result<usize> {
//     let _n = lines.next().unwrap();
//     // b'0' for black, b'1' for white, note not 0 and 1.
//     let bitstring: Vec<u8> = lines.next().unwrap().into_bytes(); // TODO: Something better than a string
//     let bitstring: Vec<bool> = bitstring.into_iter().map(|x| x == b'1').collect();
//     let q: usize = lines.next().unwrap().parse()?;
//     let bs: Vec<usize> = lines.take(q).map(|line| line.parse()).collect::<std::result::Result<_, _>>()?;

//     let mut flips = vec![false; bitstring.len()];
//     let mut pattern = vec![false; bitstring.len()];
//     flips[0] = bitstring[0];
//     pattern[0] = bitstring[0];
//     let mut solved_to = 1;

//     while solved_to < bitstring.len() {
//         for i in solved_to.. 2 * solved_to {
//             pattern[i] = pattern[i / 2];
//         }

//         for i in 0.. solved_to {
//             if solved_to + i >= bitstring.len() {
//                 break
//             }
//             pattern[solved_to + i] = pattern[i];
//             let current = bitstring[solved_to + i] ^ pattern[solved_to + i];
//             flips[solved_to + i] = current;
//             if current {
//                 pattern[solved_to + i] = !pattern[solved_to + i]; 
//             }
//         }

//         solved_to *= 2;
//     }

//     {
//         let mut flips = flips.clone();
//         for &i in &flips {
//             if i {
//                 eprint!("1")
//             } else {
//                 eprint!("0");
//             }
//         }
//         eprintln!("");
//     }

//     for i in bs {
//         flips[i - 1] = !flips[i - 1];
//     }
//     for &i in &flips {
//         if i {
//             eprint!("1")
//         } else {
//             eprint!("0");
//         }
//     }
//     eprintln!("");
//     // eprintln!("flips: {flips:?}");

//     let ret = flips.into_iter().filter(|&b| b).count();


//     Ok(ret)
// }
// [Input] 1s at positions a_1, ..., a_k (sorted order)

// Assume a_1 = 1
// All bits excepts the perfect squares gets flipped (marked with 1 for now)
// repeat the same argument at position a_2, ..., a_k
// Negate everything 

// let a_s = indicies of all the 1s in bitstring.
// let flipped = [0; _]
// for a in a_s:
//      flipped ^= array::from_fn(|j: usize| if (j - a_2 + 1) is a perfect square { 1 } else { 0 })
//
// where flipped[j] = 1, flip bit j to transform bitstring to all 0s.

// 000100000000
// a_1 = 4
// flipped = [0, 0, 0, is zero a perfect square?, 1]

// 10011000 < OG
// a_1 = 1
// flipped = [0,0,0,1,0,0,0,0,0,1] = array::from_fn(|j: usize| if (j - a_1 + 1) is a perfect square { 1 } else { 0 })
// a_2 = 4
// flipped ^= array::from_fn(|j: usize| if (j - a_2 + 1) is a perfect square { 1 } else { 0 })
// repeat for all a_i
// flip everything 

//11111111 < US 1
//01010101 < US 01
//00100100 < US 001
//00000000 < US 0000 (perfect square)
//00000100 < US 00001


// bitstring
// 00000100000000000000001000000011000000
// flips
// 10100011000000001000000010110100100101
// After bs
// 00000100000001000000011000000011000001

// 00000100000100000100000100000100000100

// 00000100   1
//         00000100

// 1
// 11
// 1111

// 01
// 0101


// abcd efgh   ijkl mnop
// 1234 5678   9
// 9  -> 1 3
// 10 -> 2 5 


// Timestep t, at which point we've only flipped bits with index <= t (and their multiples)
// i.e. Given flips[i] where i <= t is arbitrary (flips[i] where i > t is 0)
// Bitstring, starting at all 0s
// Claim: For p prime, at timestep t < p, bitstring[p] = flips[1]

// Pattern holds for each i the number of flips of that bit mod 2.
// Solving for j
// for each k such that i * k = j, we can propogate flips]i] number of flips forwards.
// except we double count... because, say, k = 4 we also get those flips from k' = 2 i' = 2*i
// Count the number of prime factors. Then
// If 1 prime factors were missing when we flipped the bit initially, we are good.
// If 2 prime factors were missing when we flipped the bit initially. We count it
// once from the original, once from adding in each other those prime factors, thus we count correctly.
// If 3 prime factors were missing when...
// Once initialy, three times adding one each, three times adding two each... mirraciously things still cancel
// We're going to get some sort of n choose k pyramid thing here.
// Wait, we have a flips array
// for i in 0.. n/2, if i % j == 0, the flip propogates forwards... except that takes too long. 








// for p prime (NOT PRIME ACTUALLY PERFECT SQUARES), nothing should be able to set it...
