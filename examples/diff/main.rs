extern crate fast_generated_dct;

use std::{f32, time::Instant};

mod rand;

fn dct_function(
    src: &[f32],
    dst: &mut [f32],
    dct_table: &[f32],
    alpha_table: &[f32],
    block_size: usize,
    block: f32,
) {
    for v in 0..block_size {
        for u in 0..block_size {
            let mut sum = 0.0f32;
            for y in 0..block_size {
                for x in 0..block_size {
                    let xu = dct_table[x * block_size + u];
                    let yv = dct_table[y * block_size + v];

                    let index = y * block_size + x;
                    sum += src[index] * xu * yv;
                }
            }

            let index = v * block_size + u;
            dst[index] = alpha_table[index] * sum * block;
        }
    }
}

fn inverse_dct_function(
    src: &[f32],
    dst: &mut [f32],
    dct_table: &[f32],
    alpha_table: &[f32],
    block_size: usize,
    block: f32,
) {
    for y in 0..block_size {
        for x in 0..block_size {
            let mut sum = 0.0f32;
            for v in 0..block_size {
                for u in 0..block_size {
                    let xu = dct_table[x * block_size + u];
                    let yv = dct_table[y * block_size + v];

                    let index = v * block_size + u;
                    sum += alpha_table[index] * src[index] * xu * yv;
                }
            }

            let index = y * block_size + x;
            dst[index] = sum * block;
        }
    }
}

fn check_output(src: &[f32], ref_: &[f32], out: &[f32], block_size: usize) {
    let mut src_max = 0.0f32;
    let mut ref_max = 0.0f32;
    let mut out_max = 0.0f32;

    let block_size_full = block_size * block_size;

    for i in 0..block_size_full {
        let src_abs = src[i].abs();
        if src_abs > src_max {
            src_max = src_abs;
        }
        let ref_abs = ref_[i].abs();
        if ref_abs > ref_max {
            ref_max = ref_abs;
        }
        let out_abs = out[i].abs();
        if out_abs > out_max {
            out_max = out_abs;
        }
    }

    let src_digits = src_max.log10() as usize + 9;
    let ref_digits = ref_max.log10() as usize + 9;
    let out_digits = out_max.log10() as usize + 9;

    let index_digits = (block_size_full as f32).log10() as usize + 1;

    let mut diff_sum = 0.0f32;
    let mut diff_percentage_sum = 0.0f32;

    let print = false;

    for i in 0..block_size_full {
        let diff = (ref_[i] - out[i]).abs();
        let average = (ref_[i].abs() + out[i].abs()) / 2.0f32;
        let diff_percentage = (diff / average).abs() * 100.0f32;

        if print {
            println!(
                "index: {1:>0$} src: {3:>2$.6} ref: {5:>4$.6} out: {7:>6$.6}  diff: {8:.6} {9:.6}%",
                index_digits,
                i,
                src_digits,
                src[i],
                ref_digits,
                ref_[i],
                out_digits,
                out[i],
                diff,
                diff_percentage
            );
        }

        diff_sum += diff;
        diff_percentage_sum += diff_percentage;
    }

    println!(
        "accumulated diff: {:.6} {:.6}%\n",
        diff_sum / block_size_full as f32,
        diff_percentage_sum / block_size_full as f32
    );
}

type FunctionDct = fn(&[f32], &mut [f32]);

fn main() {
    let functions_fast_dct: Vec<FunctionDct> = vec![
        fast_generated_dct::dct2::fast_dct,
        fast_generated_dct::dct4::fast_dct,
        fast_generated_dct::dct8::fast_dct,
        fast_generated_dct::dct16::fast_dct,
        fast_generated_dct::dct32::fast_dct,
        fast_generated_dct::dct64::fast_dct,
        fast_generated_dct::dct128::fast_dct,
        fast_generated_dct::dct256::fast_dct,
        fast_generated_dct::dct512::fast_dct,
    ];
    let functions_fast_idct: Vec<FunctionDct> = vec![
        fast_generated_dct::dct2::fast_idct,
        fast_generated_dct::dct4::fast_idct,
        fast_generated_dct::dct8::fast_idct,
        fast_generated_dct::dct16::fast_idct,
        fast_generated_dct::dct32::fast_idct,
        fast_generated_dct::dct64::fast_idct,
        fast_generated_dct::dct128::fast_idct,
        fast_generated_dct::dct256::fast_idct,
        fast_generated_dct::dct512::fast_idct,
    ];

    let mut rng = rand::Rand::new(0xDEADBEEF);

    let block_size_max = 1 << functions_fast_dct.len();

    let mut dct_src_max = vec![0.0f32; block_size_max * block_size_max];
    for i in 0..(block_size_max * block_size_max) {
        dct_src_max[i] = rng.gen_range(0, 255) as f32;
    }

    for index in 0..functions_fast_dct.len() {
        let block_size = 1 << (index + 1);
        let block_size_full = block_size * block_size;
        let block = 2.0f32 / block_size as f32;

        let mut dct_src = vec![0.0f32; block_size_full];
        for y in 0..block_size {
            for x in 0..block_size {
                dct_src[y * block_size + x] = dct_src_max[y * block_size_max + x];
            }
        }

        let mut dct_table = vec![0.0f32; block_size_full];
        let mut alpha_table = vec![0.0f32; block_size_full];

        let now = Instant::now();
        for y in 0..block_size {
            for x in 0..block_size {
                dct_table[y * block_size + x] =
                    ((2.0f32 * y as f32 + 1.0f32) * x as f32 * 3.141592f32
                        / (2.0f32 * block_size as f32))
                        .cos();
            }
        }
        for y in 0..block_size {
            for x in 0..block_size {
                alpha_table[y * block_size + x] = if y == 0 {
                    f32::consts::FRAC_1_SQRT_2
                } else {
                    1.0f32
                } * if x == 0 {
                    f32::consts::FRAC_1_SQRT_2
                } else {
                    1.0f32
                };
            }
        }
        let dct_alpha_table_elapsed = now.elapsed();

        let mut ref_dct = vec![0.0f32; block_size_full];
        let mut ref_idct = vec![0.0f32; block_size_full];
        let mut fast_dct = vec![0.0f32; block_size_full];
        let mut fast_idct = vec![0.0f32; block_size_full];

        println!("Generating {0}x{0}", block_size);

        println!("DCT");

        let now = Instant::now();
        dct_function(
            &dct_src,
            &mut ref_dct,
            &dct_table,
            &alpha_table,
            block_size,
            block,
        );
        let ref_dct_elapsed = now.elapsed();

		println!("IDCT");

        let now = Instant::now();
        inverse_dct_function(
            &ref_dct,
            &mut ref_idct,
            &dct_table,
            &alpha_table,
            block_size,
            block,
        );
        let ref_idct_elapsed = now.elapsed();

        println!("fast DCT");

        let now = Instant::now();
        functions_fast_dct[index](&dct_src, &mut fast_dct);
        let fast_dct_elapsed = now.elapsed();

        println!("fast IDCT");

        let now = Instant::now();
        functions_fast_idct[index](&fast_dct, &mut fast_idct);
        let fast_idct_elapsed = now.elapsed();

        println!("Finished\n");

        println!(
            "DCT total time: {:.6?}",
            ref_dct_elapsed + dct_alpha_table_elapsed
        );
        println!("fast DCT total time: {:.6?}\n", fast_dct_elapsed);

        check_output(&dct_src, &ref_dct, &fast_dct, block_size);

        println!(
            "IDCT total time: {:.6?}",
            ref_idct_elapsed + dct_alpha_table_elapsed
        );
        println!("fast IDCT total time: {:.6?}\n", fast_idct_elapsed);

        check_output(&ref_dct, &ref_idct, &fast_idct, block_size);
    }
}
