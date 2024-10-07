
use std::iter;
use std::vec::IntoIter;

pub fn _parse_csv<R: std::io::BufRead> (src:R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines().map(|line| {
        line.map(|line| {
            line.split(",").map(|entry| {
                String::from(entry.trim())
            }).collect()
        })
    }).collect()
}

pub fn _parse_csv_explicit_trait_params(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines().map(|line| {
        line.map(|line| {
            line.split(",").map(|entry| 
                { String::from(entry.trim())}).collect()
        })
    }).collect()
}

pub fn _combine_vec_explicit_return_type(
    v:Vec<u32>,
    u:Vec<u32>
) -> iter::Cycle<iter::Chain<IntoIter<u32>, IntoIter<u32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

pub fn _combine_vecs (
    v:Vec<i32>,
    z:Vec<i32>
) ->  impl Iterator<Item = i32> {
    v.into_iter().chain(z.into_iter()).cycle()
}