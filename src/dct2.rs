// GENERATED CODE
// Fast DCT IDCT 2x2

#![allow(non_snake_case)]

#[inline(always)]
fn fast_dct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_0 = 0.707107f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..2 {
		let s_0 = src[(0 * stridea) + src_index];
		let s_1 = src[(1 * stridea) + src_index];

		out[(0 * stridea) + out_index] = v_0 * (s_0 + s_1);
		out[(1 * stridea) + out_index] = v_0 * (s_0 - s_1);

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_dct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 2 * 2];
	fast_dct_1d(src, &mut tmp, 1, 2);
	fast_dct_1d(&tmp, out, 2, 1);
}

#[inline(always)]
fn fast_idct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_0 = 0.707107f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..2 {
		let s_0 = src[(0 * stridea) + src_index];
		let s_1 = src[(1 * stridea) + src_index];

		out[(0 * stridea) + out_index] = v_0 * (s_0 + s_1);
		out[(1 * stridea) + out_index] = v_0 * (s_0 - s_1);

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_idct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 2 * 2];
	fast_idct_1d(src, &mut tmp, 1, 2);
	fast_idct_1d(&tmp, out, 2, 1);
}