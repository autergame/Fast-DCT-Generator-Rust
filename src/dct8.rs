// GENERATED CODE
// Fast DCT IDCT 8x8

#![allow(non_snake_case)]

#[inline(always)]
fn fast_dct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_0 = -0.785695f32;
	let v_1 =  0.191342f32;
	let v_2 =  0.275899f32;
	let v_3 =  0.353553f32;
	let v_4 =  0.461940f32;
	let v_5 =  0.707107f32;
	let v_6 =  0.785695f32;
	let v_7 =  1.175876f32;
	let v_8 =  1.387040f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..8 {
		let s_0 = src[(0 * stridea) + src_index];
		let s_1 = src[(1 * stridea) + src_index];
		let s_2 = src[(2 * stridea) + src_index];
		let s_3 = src[(3 * stridea) + src_index];
		let s_4 = src[(4 * stridea) + src_index];
		let s_5 = src[(5 * stridea) + src_index];
		let s_6 = src[(6 * stridea) + src_index];
		let s_7 = src[(7 * stridea) + src_index];

		let x_00 = s_0 + s_7;
		let x_01 = s_1 + s_6;
		let x_02 = s_2 + s_5;
		let x_03 = s_3 + s_4;
		let x_04 = s_0 - s_7;
		let x_05 = s_1 - s_6;
		let x_06 = s_2 - s_5;
		let x_07 = s_3 - s_4;
		let x_08 = x_00 + x_03;
		let x_09 = x_01 + x_02;
		let x_0A = x_00 - x_03;
		let x_0B = x_01 - x_02;
		let x_0C = v_8 * x_04 + v_2 * x_07;
		let x_0D = v_7 * x_05 + v_6 * x_06;
		let x_0E = v_0 * x_05 + v_7 * x_06;
		let x_0F = v_2 * x_04 - v_8 * x_07;
		let x_10 = v_3 * (x_0C - x_0D);
		let x_11 = v_3 * (x_0E - x_0F);

		out[(0 * stridea) + out_index] = v_3 * (x_08 + x_09);
		out[(1 * stridea) + out_index] = v_3 * (x_0C + x_0D);
		out[(2 * stridea) + out_index] = v_4 * x_0A + v_1 * x_0B;
		out[(3 * stridea) + out_index] = v_5 * (x_10 - x_11);
		out[(4 * stridea) + out_index] = v_3 * (x_08 - x_09);
		out[(5 * stridea) + out_index] = v_5 * (x_10 + x_11);
		out[(6 * stridea) + out_index] = v_1 * x_0A - v_4 * x_0B;
		out[(7 * stridea) + out_index] = v_3 * (x_0E + x_0F);

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_dct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 8 * 8];
	fast_dct_1d(src, &mut tmp, 1, 8);
	fast_dct_1d(&tmp, out, 8, 1);
}

#[inline(always)]
fn fast_idct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_0 = -0.275899f32;
	let v_1 = -0.785695f32;
	let v_2 =  0.250000f32;
	let v_3 =  0.275899f32;
	let v_4 =  0.353553f32;
	let v_5 =  0.500000f32;
	let v_6 =  0.541196f32;
	let v_7 =  0.707107f32;
	let v_8 =  0.785695f32;
	let v_9 =  1.175876f32;
	let v_A =  1.306563f32;
	let v_B =  1.387040f32;
	let v_C =  1.414214f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..8 {
		let s_0 = src[(0 * stridea) + src_index];
		let s_1 = src[(1 * stridea) + src_index];
		let s_2 = src[(2 * stridea) + src_index];
		let s_3 = src[(3 * stridea) + src_index];
		let s_4 = src[(4 * stridea) + src_index];
		let s_5 = src[(5 * stridea) + src_index];
		let s_6 = src[(6 * stridea) + src_index];
		let s_7 = src[(7 * stridea) + src_index];

		let x_00 = v_C * s_0;
		let x_01 = v_B * s_1 + v_3 * s_7;
		let x_02 = v_A * s_2 + v_6 * s_6;
		let x_03 = v_9 * s_3 + v_8 * s_5;
		let x_04 = v_C * s_4;
		let x_05 = v_1 * s_3 + v_9 * s_5;
		let x_06 = v_6 * s_2 - v_A * s_6;
		let x_07 = v_0 * s_1 + v_B * s_7;
		let x_09 = x_00 + x_04;
		let x_0A = x_01 + x_03;
		let x_0B = v_C * x_02;
		let x_0C = x_00 - x_04;
		let x_0D = x_01 - x_03;
		let x_0E = v_4 * (x_09 - x_0B);
		let x_0F = v_4 * (x_0C + x_0D);
		let x_10 = v_4 * (x_0C - x_0D);
		let x_11 = v_C * x_06;
		let x_12 = x_05 + x_07;
		let x_13 = x_05 - x_07;
		let x_14 = v_4 * (x_11 + x_12);
		let x_15 = v_4 * (x_11 - x_12);
		let x_16 = v_5 * x_13;
		let x_08 = -x_15;

		out[(0 * stridea) + out_index] = v_2 * (x_09 + x_0B) + v_4 * x_0A;
		out[(1 * stridea) + out_index] = v_7 * (x_0F - x_08);
		out[(2 * stridea) + out_index] = v_7 * (x_0F + x_08);
		out[(3 * stridea) + out_index] = v_7 * (x_0E + x_16);
		out[(4 * stridea) + out_index] = v_7 * (x_0E - x_16);
		out[(5 * stridea) + out_index] = v_7 * (x_10 - x_14);
		out[(6 * stridea) + out_index] = v_7 * (x_10 + x_14);
		out[(7 * stridea) + out_index] = v_2 * (x_09 + x_0B) - v_4 * x_0A;

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_idct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 8 * 8];
	fast_idct_1d(src, &mut tmp, 1, 8);
	fast_idct_1d(&tmp, out, 8, 1);
}