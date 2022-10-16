// GENERATED CODE
// Fast DCT IDCT 16x16

#![allow(non_snake_case)]

#[inline(always)]
fn fast_dct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_00 = -0.410525f32;
	let v_01 = -0.785695f32;
	let v_02 = -0.897168f32;
	let v_03 =  0.135299f32;
	let v_04 =  0.138617f32;
	let v_05 =  0.250000f32;
	let v_06 =  0.275899f32;
	let v_07 =  0.326641f32;
	let v_08 =  0.410525f32;
	let v_09 =  0.666656f32;
	let v_0A =  0.707107f32;
	let v_0B =  0.785695f32;
	let v_0C =  0.897168f32;
	let v_0D =  1.093202f32;
	let v_0E =  1.175876f32;
	let v_0F =  1.247225f32;
	let v_10 =  1.353318f32;
	let v_11 =  1.387040f32;
	let v_12 =  1.407404f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..16 {
		let s_00 = src[( 0 * stridea) + src_index];
		let s_01 = src[( 1 * stridea) + src_index];
		let s_02 = src[( 2 * stridea) + src_index];
		let s_03 = src[( 3 * stridea) + src_index];
		let s_04 = src[( 4 * stridea) + src_index];
		let s_05 = src[( 5 * stridea) + src_index];
		let s_06 = src[( 6 * stridea) + src_index];
		let s_07 = src[( 7 * stridea) + src_index];
		let s_08 = src[( 8 * stridea) + src_index];
		let s_09 = src[( 9 * stridea) + src_index];
		let s_0A = src[(10 * stridea) + src_index];
		let s_0B = src[(11 * stridea) + src_index];
		let s_0C = src[(12 * stridea) + src_index];
		let s_0D = src[(13 * stridea) + src_index];
		let s_0E = src[(14 * stridea) + src_index];
		let s_0F = src[(15 * stridea) + src_index];

		let x_00 = s_00 + s_0F;
		let x_01 = s_01 + s_0E;
		let x_02 = s_02 + s_0D;
		let x_03 = s_03 + s_0C;
		let x_04 = s_04 + s_0B;
		let x_05 = s_05 + s_0A;
		let x_06 = s_06 + s_09;
		let x_07 = s_07 + s_08;
		let x_08 = s_00 - s_0F;
		let x_09 = s_01 - s_0E;
		let x_0A = s_02 - s_0D;
		let x_0B = s_03 - s_0C;
		let x_0C = s_04 - s_0B;
		let x_0D = s_05 - s_0A;
		let x_0E = s_06 - s_09;
		let x_0F = s_07 - s_08;
		let x_10 = x_00 + x_07;
		let x_11 = x_01 + x_06;
		let x_12 = x_02 + x_05;
		let x_13 = x_03 + x_04;
		let x_14 = x_00 - x_07;
		let x_15 = x_01 - x_06;
		let x_16 = x_02 - x_05;
		let x_17 = x_03 - x_04;
		let x_18 = x_10 + x_13;
		let x_19 = x_11 + x_12;
		let x_1A = x_10 - x_13;
		let x_1B = x_11 - x_12;
		let x_1C = v_11 * x_14 + v_06 * x_17;
		let x_1D = v_0E * x_15 + v_0B * x_16;
		let x_1E = v_01 * x_15 + v_0E * x_16;
		let x_1F = v_06 * x_14 - v_11 * x_17;
		let x_20 = v_05 * (x_1C - x_1D);
		let x_21 = v_05 * (x_1E - x_1F);
		let x_22 = v_12 * x_08 + v_04 * x_0F;
		let x_23 = v_10 * x_09 + v_08 * x_0E;
		let x_24 = v_0F * x_0A + v_09 * x_0D;
		let x_25 = v_0D * x_0B + v_0C * x_0C;
		let x_26 = v_02 * x_0B + v_0D * x_0C;
		let x_27 = v_09 * x_0A - v_0F * x_0D;
		let x_28 = v_00 * x_09 + v_10 * x_0E;
		let x_29 = v_04 * x_08 - v_12 * x_0F;
		let x_2A = x_22 + x_25;
		let x_2B = x_23 + x_24;
		let x_2C = x_22 - x_25;
		let x_2D = x_23 - x_24;
		let x_2E = v_05 * (x_2A - x_2B);
		let x_2F = v_07 * x_2C + v_03 * x_2D;
		let x_30 = v_03 * x_2C - v_07 * x_2D;
		let x_31 = x_26 + x_29;
		let x_32 = x_27 + x_28;
		let x_33 = x_26 - x_29;
		let x_34 = x_27 - x_28;
		let x_35 = v_05 * (x_31 - x_32);
		let x_36 = v_07 * x_33 + v_03 * x_34;
		let x_37 = v_03 * x_33 - v_07 * x_34;

		out[( 0 * stridea) + out_index] = v_05 * (x_18 + x_19);
		out[( 1 * stridea) + out_index] = v_05 * (x_2A + x_2B);
		out[( 2 * stridea) + out_index] = v_05 * (x_1C + x_1D);
		out[( 3 * stridea) + out_index] = v_0A * (x_2F - x_37);
		out[( 4 * stridea) + out_index] = v_07 * x_1A + v_03 * x_1B;
		out[( 5 * stridea) + out_index] = v_0A * (x_2F + x_37);
		out[( 6 * stridea) + out_index] = v_0A * (x_20 - x_21);
		out[( 7 * stridea) + out_index] = v_0A * (x_2E + x_35);
		out[( 8 * stridea) + out_index] = v_05 * (x_18 - x_19);
		out[( 9 * stridea) + out_index] = v_0A * (x_2E - x_35);
		out[(10 * stridea) + out_index] = v_0A * (x_20 + x_21);
		out[(11 * stridea) + out_index] = v_0A * (x_30 - x_36);
		out[(12 * stridea) + out_index] = v_03 * x_1A - v_07 * x_1B;
		out[(13 * stridea) + out_index] = v_0A * (x_30 + x_36);
		out[(14 * stridea) + out_index] = v_05 * (x_1E + x_1F);
		out[(15 * stridea) + out_index] = v_05 * (x_31 + x_32);

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_dct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 16 * 16];
	fast_dct_1d(src, &mut tmp, 1, 16);
	fast_dct_1d(&tmp, out, 16, 1);
}

#[inline(always)]
fn fast_idct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_00 = -0.138617f32;
	let v_01 = -0.410525f32;
	let v_02 = -0.541196f32;
	let v_03 = -0.666656f32;
	let v_04 = -0.897168f32;
	let v_05 =  0.138617f32;
	let v_06 =  0.176777f32;
	let v_07 =  0.250000f32;
	let v_08 =  0.275899f32;
	let v_09 =  0.353553f32;
	let v_0A =  0.410525f32;
	let v_0B =  0.541196f32;
	let v_0C =  0.666656f32;
	let v_0D =  0.707107f32;
	let v_0E =  0.785695f32;
	let v_0F =  0.897168f32;
	let v_10 =  1.093202f32;
	let v_11 =  1.175876f32;
	let v_12 =  1.247225f32;
	let v_13 =  1.306563f32;
	let v_14 =  1.353318f32;
	let v_15 =  1.387040f32;
	let v_16 =  1.407404f32;
	let v_17 =  1.414214f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..16 {
		let s_00 = src[( 0 * stridea) + src_index];
		let s_01 = src[( 1 * stridea) + src_index];
		let s_02 = src[( 2 * stridea) + src_index];
		let s_03 = src[( 3 * stridea) + src_index];
		let s_04 = src[( 4 * stridea) + src_index];
		let s_05 = src[( 5 * stridea) + src_index];
		let s_06 = src[( 6 * stridea) + src_index];
		let s_07 = src[( 7 * stridea) + src_index];
		let s_08 = src[( 8 * stridea) + src_index];
		let s_09 = src[( 9 * stridea) + src_index];
		let s_0A = src[(10 * stridea) + src_index];
		let s_0B = src[(11 * stridea) + src_index];
		let s_0C = src[(12 * stridea) + src_index];
		let s_0D = src[(13 * stridea) + src_index];
		let s_0E = src[(14 * stridea) + src_index];
		let s_0F = src[(15 * stridea) + src_index];

		let x_00 = v_17 * s_00;
		let x_01 = v_16 * s_01 + v_05 * s_0F;
		let x_02 = v_15 * s_02 + v_08 * s_0E;
		let x_03 = v_14 * s_03 + v_0A * s_0D;
		let x_04 = v_13 * s_04 + v_0B * s_0C;
		let x_05 = v_12 * s_05 + v_0C * s_0B;
		let x_06 = v_11 * s_06 + v_0E * s_0A;
		let x_07 = v_10 * s_07 + v_0F * s_09;
		let x_08 = v_17 * s_08;
		let x_09 = v_04 * s_07 + v_10 * s_09;
		let x_0A = v_0E * s_06 - v_11 * s_0A;
		let x_0B = v_03 * s_05 + v_12 * s_0B;
		let x_0C = v_0B * s_04 - v_13 * s_0C;
		let x_0D = v_01 * s_03 + v_14 * s_0D;
		let x_0E = v_08 * s_02 - v_15 * s_0E;
		let x_0F = v_00 * s_01 + v_16 * s_0F;
		let x_12 = x_00 + x_08;
		let x_13 = x_01 + x_07;
		let x_14 = x_02 + x_06;
		let x_15 = x_03 + x_05;
		let x_16 = v_17 * x_04;
		let x_17 = x_00 - x_08;
		let x_18 = x_01 - x_07;
		let x_19 = x_02 - x_06;
		let x_1A = x_03 - x_05;
		let x_1D = x_12 + x_16;
		let x_1E = x_13 + x_15;
		let x_1F = v_17 * x_14;
		let x_20 = x_12 - x_16;
		let x_21 = x_13 - x_15;
		let x_22 = v_07 * (x_1D - x_1F);
		let x_23 = v_07 * (x_20 + x_21);
		let x_24 = v_07 * (x_20 - x_21);
		let x_25 = v_17 * x_17;
		let x_26 = v_13 * x_18 + v_0B * x_1A;
		let x_27 = v_17 * x_19;
		let x_28 = v_02 * x_18 + v_13 * x_1A;
		let x_29 = v_06 * (x_25 + x_27) + v_07 * x_26;
		let x_2A = v_07 * (x_25 - x_27);
		let x_2B = v_06 * (x_25 + x_27) - v_07 * x_26;
		let x_2C = v_09 * x_28;
		let x_1B = v_0D * (x_2A - x_2C);
		let x_1C = v_0D * (x_2A + x_2C);
		let x_2D = v_17 * x_0C;
		let x_2E = x_0B + x_0D;
		let x_2F = x_0A + x_0E;
		let x_30 = x_09 + x_0F;
		let x_31 = x_09 - x_0F;
		let x_32 = x_0A - x_0E;
		let x_33 = x_0B - x_0D;
		let x_37 = v_17 * x_2D;
		let x_38 = v_13 * x_2E + v_0B * x_30;
		let x_39 = v_17 * x_2F;
		let x_3A = v_02 * x_2E + v_13 * x_30;
		let x_3B = v_06 * (x_37 + x_39) + v_07 * x_38;
		let x_3C = v_07 * (x_37 - x_39);
		let x_3D = v_06 * (x_37 + x_39) - v_07 * x_38;
		let x_3E = v_09 * x_3A;
		let x_34 = v_0D * (x_3C - x_3E);
		let x_35 = v_0D * (x_3C + x_3E);
		let x_3F = v_17 * x_32;
		let x_40 = x_31 + x_33;
		let x_41 = x_31 - x_33;
		let x_42 = v_07 * (x_3F + x_40);
		let x_43 = v_07 * (x_3F - x_40);
		let x_44 = v_09 * x_41;
		let x_36 = -x_43;
		let x_10 = -x_34;
		let x_11 = -x_3D;

		out[( 0 * stridea) + out_index] = v_06 * (x_1D + x_1F) + v_07 * x_1E;
		out[( 1 * stridea) + out_index] = v_0D * (x_29 - x_11);
		out[( 2 * stridea) + out_index] = v_0D * (x_29 + x_11);
		out[( 3 * stridea) + out_index] = v_0D * (x_23 + x_36);
		out[( 4 * stridea) + out_index] = v_0D * (x_23 - x_36);
		out[( 5 * stridea) + out_index] = v_0D * (x_1B - x_35);
		out[( 6 * stridea) + out_index] = v_0D * (x_1B + x_35);
		out[( 7 * stridea) + out_index] = v_0D * (x_22 + x_44);
		out[( 8 * stridea) + out_index] = v_0D * (x_22 - x_44);
		out[( 9 * stridea) + out_index] = v_0D * (x_1C - x_10);
		out[(10 * stridea) + out_index] = v_0D * (x_1C + x_10);
		out[(11 * stridea) + out_index] = v_0D * (x_24 + x_42);
		out[(12 * stridea) + out_index] = v_0D * (x_24 - x_42);
		out[(13 * stridea) + out_index] = v_0D * (x_2B - x_3B);
		out[(14 * stridea) + out_index] = v_0D * (x_2B + x_3B);
		out[(15 * stridea) + out_index] = v_06 * (x_1D + x_1F) - v_07 * x_1E;

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_idct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 16 * 16];
	fast_idct_1d(src, &mut tmp, 1, 16);
	fast_idct_1d(&tmp, out, 16, 1);
}