/**
 * The MIT License
 * Copyright (c) 2022 Guillem Castro
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */
use mountinfo::{MountInfo, MountPoint, ReadWrite};

fn main() {
    let mtab = MountInfo::new().unwrap();

    println!("Read-only mount points:");
    for mount_point in mtab
        .mounting_points
        .iter()
        .filter(|&mount_point| match mount_point.options.read_write {
            ReadWrite::ReadOnly => true,
            _ => false,
        })
        .collect::<Vec<&MountPoint>>()
    {
        println!("{:?}", mount_point);
    }

    println!("Read-write mount points:");
    for mount_point in mtab
        .mounting_points
        .iter()
        .filter(|&mount_point| match mount_point.options.read_write {
            ReadWrite::ReadWrite => true,
            _ => false,
        })
        .collect::<Vec<&MountPoint>>()
    {
        println!("{:?}", mount_point);
    }
}
