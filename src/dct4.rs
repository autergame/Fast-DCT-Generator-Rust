// GENERATED CODE
// Fast DCT IDCT 4x4

#![allow(non_snake_case)]

#[inline(always)]
fn fast_dct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_0 = 0.270598f32;
	let v_1 = 0.500000f32;
	let v_2 = 0.653281f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..4 {
		let s_0 = src[(0 * stridea) + src_index];
		let s_1 = src[(1 * stridea) + src_index];
		let s_2 = src[(2 * stridea) + src_index];
		let s_3 = src[(3 * stridea) + src_index];

		let x_0 = s_0 + s_3;
		let x_1 = s_1 + s_2;
		let x_2 = s_0 - s_3;
		let x_3 = s_1 - s_2;

		out[(0 * stridea) + out_index] = v_1 * (x_0 + x_1);
		out[(1 * stridea) + out_index] = v_2 * x_2 + v_0 * x_3;
		out[(2 * stridea) + out_index] = v_1 * (x_0 - x_1);
		out[(3 * stridea) + out_index] = v_0 * x_2 - v_2 * x_3;

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_dct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 4 * 4];
	fast_dct_1d(src, &mut tmp, 1, 4);
	fast_dct_1d(&tmp, out, 4, 1);
}

#[inline(always)]
fn fast_idct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_0 = -0.541196f32;
	let v_1 =  0.353553f32;
	let v_2 =  0.500000f32;
	let v_3 =  0.541196f32;
	let v_4 =  0.707107f32;
	let v_5 =  1.306563f32;
	let v_6 =  1.414214f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..4 {
		let s_0 = src[(0 * stridea) + src_index];
		let s_1 = src[(1 * stridea) + src_index];
		let s_2 = src[(2 * stridea) + src_index];
		let s_3 = src[(3 * stridea) + src_index];

		let x_0 = v_6 * s_0;
		let x_1 = v_5 * s_1 + v_3 * s_3;
		let x_2 = v_6 * s_2;
		let x_3 = v_0 * s_1 + v_5 * s_3;
		let x_4 = v_2 * (x_0 - x_2);
		let x_5 = v_4 * x_3;

		out[(0 * stridea) + out_index] = v_1 * (x_0 + x_2) + v_2 * x_1;
		out[(1 * stridea) + out_index] = v_4 * (x_4 - x_5);
		out[(2 * stridea) + out_index] = v_4 * (x_4 + x_5);
		out[(3 * stridea) + out_index] = v_1 * (x_0 + x_2) - v_2 * x_1;

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_idct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 4 * 4];
	fast_idct_1d(src, &mut tmp, 1, 4);
	fast_idct_1d(&tmp, out, 4, 1);
}