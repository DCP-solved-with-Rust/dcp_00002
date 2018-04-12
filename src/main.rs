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

    let product = nums.iter().fold(1i32, |prod, &val| prod * val);

    for num in nums.iter()
    {
        print!("{} ", product / num);
    }

    print!("\n");

    /*
     * To answer the question "what if we couldn't use division;
     * then we'd use a double loop where we recalculated the product
     * each time but we skipped the current number.
     *
     * Also, if they hadn't asked that question, that's what I would
     * have done -- I wouldn't have thought of calculating the product
     * and then dividing by the current number if they didn't mention
     * division.
     */
}
