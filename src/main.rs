/*
 * Copyright (c) 2018 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use std::vec::Vec;

use std::io;
use std::io::prelude::*;

fn get_products (nums: Vec<i32>) -> Vec<i32>
{
    let mut prod_left_side = Vec::with_capacity(nums.len());

    let mut prod = 1;
    for &num in nums.iter()
    {
        prod_left_side.push(prod);
        prod *= num;
    }

    let mut prod_right_side = vec![0; nums.len()];

    prod = 1;
    for (&num, mut prod_i) in nums.iter().rev().zip(prod_right_side.iter_mut().rev())
    {
        *prod_i = prod;
        prod *= num;
    }

    let mut results = Vec::with_capacity(nums.len());

    for (&left, &right) in prod_left_side.iter().zip(prod_right_side.iter())
    {
        results.push(left * right);
    }

    return results;
}

fn main ()
{
    let stdin = io::stdin();

    let mut nums = Vec::new();

    for line in stdin.lock().lines()
    {
        let mut curr_inputs: Vec<i32> = line.unwrap().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        nums.append(&mut curr_inputs);
    }

    if nums.len() < 2
    {
        panic!("Need at least two numbers!");
    }

    let results = get_products(nums);

    for result in results.iter().take(results.len() - 1)
    {
        print!("{} ", result);
    }

    print!("{}\n", results.last().unwrap());
}
