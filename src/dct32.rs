// GENERATED CODE
// Fast DCT IDCT 32x32

#![allow(non_snake_case)]

#[inline(always)]
fn fast_dct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_00 = -0.207508f32;
	let v_01 = -0.410525f32;
	let v_02 = -0.476434f32;
	let v_03 = -0.727051f32;
	let v_04 = -0.785695f32;
	let v_05 = -0.897168f32;
	let v_06 = -0.949728f32;
	let v_07 =  0.069392f32;
	let v_08 =  0.095671f32;
	let v_09 =  0.138617f32;
	let v_0A =  0.176777f32;
	let v_0B =  0.207508f32;
	let v_0C =  0.230970f32;
	let v_0D =  0.275899f32;
	let v_0E =  0.343626f32;
	let v_0F =  0.410525f32;
	let v_10 =  0.476434f32;
	let v_11 =  0.604654f32;
	let v_12 =  0.666656f32;
	let v_13 =  0.707107f32;
	let v_14 =  0.727051f32;
	let v_15 =  0.785695f32;
	let v_16 =  0.842446f32;
	let v_17 =  0.897168f32;
	let v_18 =  0.949728f32;
	let v_19 =  1.047863f32;
	let v_1A =  1.093202f32;
	let v_1B =  1.135907f32;
	let v_1C =  1.175876f32;
	let v_1D =  1.213011f32;
	let v_1E =  1.247225f32;
	let v_1F =  1.278434f32;
	let v_20 =  1.331544f32;
	let v_21 =  1.353318f32;
	let v_22 =  1.371831f32;
	let v_23 =  1.387040f32;
	let v_24 =  1.398907f32;
	let v_25 =  1.407404f32;
	let v_26 =  1.412510f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..32 {
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
		let s_10 = src[(16 * stridea) + src_index];
		let s_11 = src[(17 * stridea) + src_index];
		let s_12 = src[(18 * stridea) + src_index];
		let s_13 = src[(19 * stridea) + src_index];
		let s_14 = src[(20 * stridea) + src_index];
		let s_15 = src[(21 * stridea) + src_index];
		let s_16 = src[(22 * stridea) + src_index];
		let s_17 = src[(23 * stridea) + src_index];
		let s_18 = src[(24 * stridea) + src_index];
		let s_19 = src[(25 * stridea) + src_index];
		let s_1A = src[(26 * stridea) + src_index];
		let s_1B = src[(27 * stridea) + src_index];
		let s_1C = src[(28 * stridea) + src_index];
		let s_1D = src[(29 * stridea) + src_index];
		let s_1E = src[(30 * stridea) + src_index];
		let s_1F = src[(31 * stridea) + src_index];

		let x_00 = s_00 + s_1F;
		let x_11 = s_01 + s_1E;
		let x_12 = s_02 + s_1D;
		let x_13 = s_03 + s_1C;
		let x_14 = s_04 + s_1B;
		let x_15 = s_05 + s_1A;
		let x_16 = s_06 + s_19;
		let x_17 = s_07 + s_18;
		let x_18 = s_08 + s_17;
		let x_19 = s_09 + s_16;
		let x_1A = s_0A + s_15;
		let x_1B = s_0B + s_14;
		let x_1C = s_0C + s_13;
		let x_1D = s_0D + s_12;
		let x_1E = s_0E + s_11;
		let x_1F = s_0F + s_10;
		let x_01 = s_00 - s_1F;
		let x_02 = s_01 - s_1E;
		let x_03 = s_02 - s_1D;
		let x_04 = s_03 - s_1C;
		let x_05 = s_04 - s_1B;
		let x_06 = s_05 - s_1A;
		let x_07 = s_06 - s_19;
		let x_08 = s_07 - s_18;
		let x_09 = s_08 - s_17;
		let x_0A = s_09 - s_16;
		let x_0B = s_0A - s_15;
		let x_0C = s_0B - s_14;
		let x_0D = s_0C - s_13;
		let x_0E = s_0D - s_12;
		let x_0F = s_0E - s_11;
		let x_10 = s_0F - s_10;
		let x_62 = x_00 + x_1F;
		let x_63 = x_11 + x_1E;
		let x_64 = x_12 + x_1D;
		let x_65 = x_13 + x_1C;
		let x_66 = x_14 + x_1B;
		let x_67 = x_15 + x_1A;
		let x_68 = x_16 + x_19;
		let x_69 = x_17 + x_18;
		let x_6A = x_00 - x_1F;
		let x_6B = x_11 - x_1E;
		let x_6C = x_12 - x_1D;
		let x_6D = x_13 - x_1C;
		let x_6E = x_14 - x_1B;
		let x_6F = x_15 - x_1A;
		let x_70 = x_16 - x_19;
		let x_71 = x_17 - x_18;
		let x_72 = x_62 + x_69;
		let x_73 = x_63 + x_68;
		let x_74 = x_64 + x_67;
		let x_75 = x_65 + x_66;
		let x_76 = x_62 - x_69;
		let x_77 = x_63 - x_68;
		let x_78 = x_64 - x_67;
		let x_79 = x_65 - x_66;
		let x_7A = x_72 + x_75;
		let x_7B = x_73 + x_74;
		let x_7C = x_72 - x_75;
		let x_7D = x_73 - x_74;
		let x_7E = v_23 * x_76 + v_0D * x_79;
		let x_7F = v_1C * x_77 + v_15 * x_78;
		let x_80 = v_04 * x_77 + v_1C * x_78;
		let x_81 = v_0D * x_76 - v_23 * x_79;
		let x_82 = v_0A * (x_7E - x_7F);
		let x_83 = v_0A * (x_80 - x_81);
		let x_84 = v_25 * x_6A + v_09 * x_71;
		let x_85 = v_21 * x_6B + v_0F * x_70;
		let x_86 = v_1E * x_6C + v_12 * x_6F;
		let x_87 = v_1A * x_6D + v_17 * x_6E;
		let x_88 = v_05 * x_6D + v_1A * x_6E;
		let x_89 = v_12 * x_6C - v_1E * x_6F;
		let x_8A = v_01 * x_6B + v_21 * x_70;
		let x_8B = v_09 * x_6A - v_25 * x_71;
		let x_8C = x_84 + x_87;
		let x_8D = x_85 + x_86;
		let x_8E = x_84 - x_87;
		let x_8F = x_85 - x_86;
		let x_90 = v_0A * (x_8C - x_8D);
		let x_91 = v_0C * x_8E + v_08 * x_8F;
		let x_92 = v_08 * x_8E - v_0C * x_8F;
		let x_93 = x_88 + x_8B;
		let x_94 = x_89 + x_8A;
		let x_95 = x_88 - x_8B;
		let x_96 = x_89 - x_8A;
		let x_97 = v_0A * (x_93 - x_94);
		let x_98 = v_0C * x_95 + v_08 * x_96;
		let x_99 = v_08 * x_95 - v_0C * x_96;
		let x_20 = v_26 * x_01 + v_07 * x_10;
		let x_21 = v_24 * x_02 + v_0B * x_0F;
		let x_22 = v_22 * x_03 + v_0E * x_0E;
		let x_23 = v_20 * x_04 + v_10 * x_0D;
		let x_24 = v_1F * x_05 + v_11 * x_0C;
		let x_25 = v_1D * x_06 + v_14 * x_0B;
		let x_26 = v_1B * x_07 + v_16 * x_0A;
		let x_27 = v_19 * x_08 + v_18 * x_09;
		let x_28 = v_06 * x_08 + v_19 * x_09;
		let x_29 = v_16 * x_07 - v_1B * x_0A;
		let x_2A = v_03 * x_06 + v_1D * x_0B;
		let x_2B = v_11 * x_05 - v_1F * x_0C;
		let x_2C = v_02 * x_04 + v_20 * x_0D;
		let x_2D = v_0E * x_03 - v_22 * x_0E;
		let x_2E = v_00 * x_02 + v_24 * x_0F;
		let x_2F = v_07 * x_01 - v_26 * x_10;
		let x_30 = x_20 + x_27;
		let x_31 = x_21 + x_26;
		let x_32 = x_22 + x_25;
		let x_33 = x_23 + x_24;
		let x_34 = x_20 - x_27;
		let x_35 = x_21 - x_26;
		let x_36 = x_22 - x_25;
		let x_37 = x_23 - x_24;
		let x_3A = x_30 + x_33;
		let x_3B = x_31 + x_32;
		let x_3C = x_30 - x_33;
		let x_3D = x_31 - x_32;
		let x_3E = v_0A * (x_3A - x_3B);
		let x_3F = v_0C * x_3C + v_08 * x_3D;
		let x_40 = v_08 * x_3C - v_0C * x_3D;
		let x_41 = v_23 * x_34 + v_0D * x_37;
		let x_42 = v_1C * x_35 + v_15 * x_36;
		let x_43 = v_04 * x_35 + v_1C * x_36;
		let x_44 = v_0D * x_34 - v_23 * x_37;
		let x_45 = v_0A * (x_41 + x_42);
		let x_46 = v_0A * (x_41 - x_42);
		let x_47 = v_0A * (x_43 + x_44);
		let x_48 = v_0A * (x_43 - x_44);
		let x_38 = v_13 * (x_46 - x_48);
		let x_39 = v_13 * (x_46 + x_48);
		let x_49 = x_28 + x_2F;
		let x_4A = x_29 + x_2E;
		let x_4B = x_2A + x_2D;
		let x_4C = x_2B + x_2C;
		let x_4D = x_28 - x_2F;
		let x_4E = x_29 - x_2E;
		let x_4F = x_2A - x_2D;
		let x_50 = x_2B - x_2C;
		let x_53 = x_49 + x_4C;
		let x_54 = x_4A + x_4B;
		let x_55 = x_49 - x_4C;
		let x_56 = x_4A - x_4B;
		let x_57 = v_0A * (x_53 - x_54);
		let x_58 = v_0C * x_55 + v_08 * x_56;
		let x_59 = v_08 * x_55 - v_0C * x_56;
		let x_5A = v_23 * x_4D + v_0D * x_50;
		let x_5B = v_1C * x_4E + v_15 * x_4F;
		let x_5C = v_04 * x_4E + v_1C * x_4F;
		let x_5D = v_0D * x_4D - v_23 * x_50;
		let x_5E = v_0A * (x_5A + x_5B);
		let x_5F = v_0A * (x_5A - x_5B);
		let x_60 = v_0A * (x_5C + x_5D);
		let x_61 = v_0A * (x_5C - x_5D);
		let x_51 = v_13 * (x_5F - x_61);
		let x_52 = v_13 * (x_5F + x_61);

		out[( 0 * stridea) + out_index] = v_0A * (x_7A + x_7B);
		out[( 1 * stridea) + out_index] = v_0A * (x_3A + x_3B);
		out[( 2 * stridea) + out_index] = v_0A * (x_8C + x_8D);
		out[( 3 * stridea) + out_index] = v_13 * (x_45 - x_60);
		out[( 4 * stridea) + out_index] = v_0A * (x_7E + x_7F);
		out[( 5 * stridea) + out_index] = v_13 * (x_45 + x_60);
		out[( 6 * stridea) + out_index] = v_13 * (x_91 - x_99);
		out[( 7 * stridea) + out_index] = v_13 * (x_3F + x_59);
		out[( 8 * stridea) + out_index] = v_0C * x_7C + v_08 * x_7D;
		out[( 9 * stridea) + out_index] = v_13 * (x_3F - x_59);
		out[(10 * stridea) + out_index] = v_13 * (x_91 + x_99);
		out[(11 * stridea) + out_index] = v_13 * (x_38 - x_52);
		out[(12 * stridea) + out_index] = v_13 * (x_82 - x_83);
		out[(13 * stridea) + out_index] = v_13 * (x_38 + x_52);
		out[(14 * stridea) + out_index] = v_13 * (x_90 + x_97);
		out[(15 * stridea) + out_index] = v_13 * (x_3E + x_57);
		out[(16 * stridea) + out_index] = v_0A * (x_7A - x_7B);
		out[(17 * stridea) + out_index] = v_13 * (x_3E - x_57);
		out[(18 * stridea) + out_index] = v_13 * (x_90 - x_97);
		out[(19 * stridea) + out_index] = v_13 * (x_39 - x_51);
		out[(20 * stridea) + out_index] = v_13 * (x_82 + x_83);
		out[(21 * stridea) + out_index] = v_13 * (x_39 + x_51);
		out[(22 * stridea) + out_index] = v_13 * (x_92 - x_98);
		out[(23 * stridea) + out_index] = v_13 * (x_40 + x_58);
		out[(24 * stridea) + out_index] = v_08 * x_7C - v_0C * x_7D;
		out[(25 * stridea) + out_index] = v_13 * (x_40 - x_58);
		out[(26 * stridea) + out_index] = v_13 * (x_92 + x_98);
		out[(27 * stridea) + out_index] = v_13 * (x_47 - x_5E);
		out[(28 * stridea) + out_index] = v_0A * (x_80 + x_81);
		out[(29 * stridea) + out_index] = v_13 * (x_47 + x_5E);
		out[(30 * stridea) + out_index] = v_0A * (x_93 + x_94);
		out[(31 * stridea) + out_index] = v_0A * (x_53 + x_54);

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_dct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 32 * 32];
	fast_dct_1d(src, &mut tmp, 1, 32);
	fast_dct_1d(&tmp, out, 32, 1);
}

#[inline(always)]
fn fast_idct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
	let v_00 = -0.069392f32;
	let v_01 = -0.207508f32;
	let v_02 = -0.275899f32;
	let v_03 = -0.343626f32;
	let v_04 = -0.476434f32;
	let v_05 = -0.541196f32;
	let v_06 = -0.604654f32;
	let v_07 = -0.707107f32;
	let v_08 = -0.727051f32;
	let v_09 = -0.785695f32;
	let v_0A = -0.842446f32;
	let v_0B = -0.949728f32;
	let v_0C =  0.069392f32;
	let v_0D =  0.125000f32;
	let v_0E =  0.138617f32;
	let v_0F =  0.176777f32;
	let v_10 =  0.207508f32;
	let v_11 =  0.250000f32;
	let v_12 =  0.275899f32;
	let v_13 =  0.343626f32;
	let v_14 =  0.410525f32;
	let v_15 =  0.476434f32;
	let v_16 =  0.541196f32;
	let v_17 =  0.604654f32;
	let v_18 =  0.666656f32;
	let v_19 =  0.707107f32;
	let v_1A =  0.727051f32;
	let v_1B =  0.785695f32;
	let v_1C =  0.842446f32;
	let v_1D =  0.897168f32;
	let v_1E =  0.949728f32;
	let v_1F =  1.047863f32;
	let v_20 =  1.093202f32;
	let v_21 =  1.135907f32;
	let v_22 =  1.175876f32;
	let v_23 =  1.213011f32;
	let v_24 =  1.247225f32;
	let v_25 =  1.278434f32;
	let v_26 =  1.306563f32;
	let v_27 =  1.331544f32;
	let v_28 =  1.353318f32;
	let v_29 =  1.371831f32;
	let v_2A =  1.387040f32;
	let v_2B =  1.398907f32;
	let v_2C =  1.407404f32;
	let v_2D =  1.412510f32;
	let v_2E =  1.414214f32;

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..32 {
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
		let s_10 = src[(16 * stridea) + src_index];
		let s_11 = src[(17 * stridea) + src_index];
		let s_12 = src[(18 * stridea) + src_index];
		let s_13 = src[(19 * stridea) + src_index];
		let s_14 = src[(20 * stridea) + src_index];
		let s_15 = src[(21 * stridea) + src_index];
		let s_16 = src[(22 * stridea) + src_index];
		let s_17 = src[(23 * stridea) + src_index];
		let s_18 = src[(24 * stridea) + src_index];
		let s_19 = src[(25 * stridea) + src_index];
		let s_1A = src[(26 * stridea) + src_index];
		let s_1B = src[(27 * stridea) + src_index];
		let s_1C = src[(28 * stridea) + src_index];
		let s_1D = src[(29 * stridea) + src_index];
		let s_1E = src[(30 * stridea) + src_index];
		let s_1F = src[(31 * stridea) + src_index];

		let x_00 = v_2E * s_00;
		let x_11 = v_2D * s_01 + v_0C * s_1F;
		let x_12 = v_2C * s_02 + v_0E * s_1E;
		let x_13 = v_2B * s_03 + v_10 * s_1D;
		let x_14 = v_2A * s_04 + v_12 * s_1C;
		let x_15 = v_29 * s_05 + v_13 * s_1B;
		let x_16 = v_28 * s_06 + v_14 * s_1A;
		let x_17 = v_27 * s_07 + v_15 * s_19;
		let x_18 = v_26 * s_08 + v_16 * s_18;
		let x_19 = v_25 * s_09 + v_17 * s_17;
		let x_1A = v_24 * s_0A + v_18 * s_16;
		let x_1B = v_23 * s_0B + v_1A * s_15;
		let x_1C = v_22 * s_0C + v_1B * s_14;
		let x_1D = v_21 * s_0D + v_1C * s_13;
		let x_1E = v_20 * s_0E + v_1D * s_12;
		let x_1F = v_1F * s_0F + v_1E * s_11;
		let x_01 = v_2E * s_10;
		let x_02 = v_0B * s_0F + v_1F * s_11;
		let x_03 = v_1D * s_0E - v_20 * s_12;
		let x_04 = v_0A * s_0D + v_21 * s_13;
		let x_05 = v_1B * s_0C - v_22 * s_14;
		let x_06 = v_08 * s_0B + v_23 * s_15;
		let x_07 = v_18 * s_0A - v_24 * s_16;
		let x_08 = v_06 * s_09 + v_25 * s_17;
		let x_09 = v_16 * s_08 - v_26 * s_18;
		let x_0A = v_04 * s_07 + v_27 * s_19;
		let x_0B = v_14 * s_06 - v_28 * s_1A;
		let x_0C = v_03 * s_05 + v_29 * s_1B;
		let x_0D = v_12 * s_04 - v_2A * s_1C;
		let x_0E = v_01 * s_03 + v_2B * s_1D;
		let x_0F = v_0E * s_02 - v_2C * s_1E;
		let x_10 = v_00 * s_01 + v_2D * s_1F;
		let x_6C = x_00 + x_01;
		let x_6E = x_11 + x_1F;
		let x_6F = x_12 + x_1E;
		let x_70 = x_13 + x_1D;
		let x_71 = x_14 + x_1C;
		let x_72 = x_15 + x_1B;
		let x_73 = x_16 + x_1A;
		let x_74 = x_17 + x_19;
		let x_75 = v_2E * x_18;
		let x_76 = x_00 - x_01;
		let x_77 = x_11 - x_1F;
		let x_78 = x_12 - x_1E;
		let x_79 = x_13 - x_1D;
		let x_7A = x_14 - x_1C;
		let x_7B = x_15 - x_1B;
		let x_7C = x_16 - x_1A;
		let x_6D = x_17 - x_19;
		let x_83 = x_6C + x_75;
		let x_84 = x_6E + x_74;
		let x_85 = x_6F + x_73;
		let x_86 = x_70 + x_72;
		let x_87 = v_2E * x_71;
		let x_88 = x_6C - x_75;
		let x_89 = x_6E - x_74;
		let x_8A = x_6F - x_73;
		let x_8B = x_70 - x_72;
		let x_8E = x_83 + x_87;
		let x_8F = x_84 + x_86;
		let x_90 = v_2E * x_85;
		let x_91 = x_83 - x_87;
		let x_92 = x_84 - x_86;
		let x_93 = v_0F * (x_8E - x_90);
		let x_94 = v_0F * (x_91 + x_92);
		let x_95 = v_0F * (x_91 - x_92);
		let x_96 = v_2E * x_88;
		let x_97 = v_26 * x_89 + v_16 * x_8B;
		let x_98 = v_2E * x_8A;
		let x_99 = v_05 * x_89 + v_26 * x_8B;
		let x_9A = v_0D * (x_96 + x_98) + v_0F * x_97;
		let x_9B = v_0F * (x_96 - x_98);
		let x_9C = v_0D * (x_96 + x_98) - v_0F * x_97;
		let x_9D = v_11 * x_99;
		let x_8C = v_19 * (x_9B - x_9D);
		let x_8D = v_19 * (x_9B + x_9D);
		let x_9E = v_2E * x_76;
		let x_9F = v_12 * x_6D + v_2A * x_77;
		let x_A0 = v_26 * x_78 + v_16 * x_7C;
		let x_A1 = v_22 * x_79 + v_1B * x_7B;
		let x_A2 = v_2E * x_7A;
		let x_A3 = v_09 * x_79 + v_22 * x_7B;
		let x_A4 = v_16 * x_78 - v_26 * x_7C;
		let x_A5 = v_2A * x_6D - v_12 * x_77;
		let x_A7 = x_9E + x_A2;
		let x_A8 = x_9F + x_A1;
		let x_A9 = v_2E * x_A0;
		let x_AA = x_9E - x_A2;
		let x_AB = x_9F - x_A1;
		let x_AC = v_0D * (x_A7 + x_A9) + v_0F * x_A8;
		let x_AD = v_0F * (x_A7 - x_A9);
		let x_AE = v_0D * (x_A7 + x_A9) - v_0F * x_A8;
		let x_AF = v_0F * (x_AA + x_AB);
		let x_B0 = v_0F * (x_AA - x_AB);
		let x_B1 = v_2E * x_A4;
		let x_B2 = x_A3 + x_A5;
		let x_B3 = x_A3 - x_A5;
		let x_B4 = v_0F * (x_B1 + x_B2);
		let x_B5 = v_0F * (x_B1 - x_B2);
		let x_B6 = v_11 * x_B3;
		let x_A6 = -x_B5;
		let x_7D = v_19 * (x_AF - x_A6);
		let x_7E = v_19 * (x_AF + x_A6);
		let x_7F = v_19 * (x_AD + x_B6);
		let x_80 = v_19 * (x_AD - x_B6);
		let x_81 = v_19 * (x_B0 - x_B4);
		let x_82 = v_19 * (x_B0 + x_B4);
		let x_20 = v_2E * x_09;
		let x_21 = x_08 + x_0A;
		let x_22 = x_07 + x_0B;
		let x_23 = x_06 + x_0C;
		let x_24 = x_05 + x_0D;
		let x_25 = x_04 + x_0E;
		let x_26 = x_03 + x_0F;
		let x_27 = x_02 + x_10;
		let x_28 = x_02 - x_10;
		let x_29 = x_03 - x_0F;
		let x_2A = x_04 - x_0E;
		let x_2B = x_05 - x_0D;
		let x_2C = x_06 - x_0C;
		let x_2D = x_07 - x_0B;
		let x_2E = x_08 - x_0A;
		let x_37 = v_2E * x_20;
		let x_38 = v_2A * x_21 + v_12 * x_27;
		let x_39 = v_26 * x_22 + v_16 * x_26;
		let x_3A = v_22 * x_23 + v_1B * x_25;
		let x_3B = v_2E * x_24;
		let x_3C = v_09 * x_23 + v_22 * x_25;
		let x_3D = v_16 * x_22 - v_26 * x_26;
		let x_3E = v_02 * x_21 + v_2A * x_27;
		let x_40 = x_37 + x_3B;
		let x_41 = x_38 + x_3A;
		let x_42 = v_2E * x_39;
		let x_43 = x_37 - x_3B;
		let x_44 = x_38 - x_3A;
		let x_45 = v_0D * (x_40 + x_42) + v_0F * x_41;
		let x_46 = v_0F * (x_40 - x_42);
		let x_47 = v_0D * (x_40 + x_42) - v_0F * x_41;
		let x_48 = v_0F * (x_43 + x_44);
		let x_49 = v_0F * (x_43 - x_44);
		let x_4A = v_2E * x_3D;
		let x_4B = x_3C + x_3E;
		let x_4C = x_3C - x_3E;
		let x_4D = v_0F * (x_4A + x_4B);
		let x_4E = v_0F * (x_4A - x_4B);
		let x_4F = v_11 * x_4C;
		let x_3F = -x_4E;
		let x_2F = v_19 * (x_48 - x_3F);
		let x_30 = v_19 * (x_48 + x_3F);
		let x_31 = v_19 * (x_46 + x_4F);
		let x_32 = v_19 * (x_46 - x_4F);
		let x_33 = v_19 * (x_49 - x_4D);
		let x_34 = v_19 * (x_49 + x_4D);
		let x_50 = v_2E * x_2B;
		let x_51 = x_2A + x_2C;
		let x_52 = x_29 + x_2D;
		let x_53 = x_28 + x_2E;
		let x_54 = x_28 - x_2E;
		let x_55 = x_29 - x_2D;
		let x_56 = x_2A - x_2C;
		let x_5E = v_2E * x_50;
		let x_5F = v_26 * x_51 + v_16 * x_53;
		let x_60 = v_2E * x_52;
		let x_61 = v_05 * x_51 + v_26 * x_53;
		let x_62 = v_0D * (x_5E + x_60) + v_0F * x_5F;
		let x_63 = v_0F * (x_5E - x_60);
		let x_64 = v_0D * (x_5E + x_60) - v_0F * x_5F;
		let x_65 = v_11 * x_61;
		let x_57 = v_19 * (x_63 - x_65);
		let x_58 = v_19 * (x_63 + x_65);
		let x_66 = v_2E * x_55;
		let x_67 = x_54 + x_56;
		let x_68 = x_54 - x_56;
		let x_69 = v_0F * (x_66 + x_67);
		let x_6A = v_0F * (x_66 - x_67);
		let x_6B = v_11 * x_68;
		let x_59 = -x_6A;
		let x_35 = -x_57;
		let x_36 = -x_64;
		let x_5A = -x_2F;
		let x_5B = -x_31;
		let x_5C = -x_33;
		let x_5D = -x_47;

		out[( 0 * stridea) + out_index] = v_0D * (x_8E + x_90) + v_0F * x_8F;
		out[( 1 * stridea) + out_index] = v_07 * x_5D + v_19 * x_AC;
		out[( 2 * stridea) + out_index] = v_19 * (x_5D + x_AC);
		out[( 3 * stridea) + out_index] = v_19 * (x_36 + x_9A);
		out[( 4 * stridea) + out_index] = v_07 * x_36 + v_19 * x_9A;
		out[( 5 * stridea) + out_index] = v_07 * x_34 + v_19 * x_7D;
		out[( 6 * stridea) + out_index] = v_19 * (x_34 + x_7D);
		out[( 7 * stridea) + out_index] = v_19 * (x_59 + x_94);
		out[( 8 * stridea) + out_index] = v_07 * x_59 + v_19 * x_94;
		out[( 9 * stridea) + out_index] = v_07 * x_5C + v_19 * x_7E;
		out[(10 * stridea) + out_index] = v_19 * (x_5C + x_7E);
		out[(11 * stridea) + out_index] = v_19 * (x_58 + x_8C);
		out[(12 * stridea) + out_index] = v_07 * x_58 + v_19 * x_8C;
		out[(13 * stridea) + out_index] = v_07 * x_32 + v_19 * x_7F;
		out[(14 * stridea) + out_index] = v_19 * (x_32 + x_7F);
		out[(15 * stridea) + out_index] = v_19 * (x_6B + x_93);
		out[(16 * stridea) + out_index] = v_07 * x_6B + v_19 * x_93;
		out[(17 * stridea) + out_index] = v_07 * x_5B + v_19 * x_80;
		out[(18 * stridea) + out_index] = v_19 * (x_5B + x_80);
		out[(19 * stridea) + out_index] = v_19 * (x_35 + x_8D);
		out[(20 * stridea) + out_index] = v_07 * x_35 + v_19 * x_8D;
		out[(21 * stridea) + out_index] = v_07 * x_30 + v_19 * x_81;
		out[(22 * stridea) + out_index] = v_19 * (x_30 + x_81);
		out[(23 * stridea) + out_index] = v_19 * (x_69 + x_95);
		out[(24 * stridea) + out_index] = v_07 * x_69 + v_19 * x_95;
		out[(25 * stridea) + out_index] = v_07 * x_5A + v_19 * x_82;
		out[(26 * stridea) + out_index] = v_19 * (x_5A + x_82);
		out[(27 * stridea) + out_index] = v_19 * (x_62 + x_9C);
		out[(28 * stridea) + out_index] = v_07 * x_62 + v_19 * x_9C;
		out[(29 * stridea) + out_index] = v_07 * x_45 + v_19 * x_AE;
		out[(30 * stridea) + out_index] = v_19 * (x_45 + x_AE);
		out[(31 * stridea) + out_index] = v_0D * (x_8E + x_90) - v_0F * x_8F;

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_idct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; 32 * 32];
	fast_idct_1d(src, &mut tmp, 1, 32);
	fast_idct_1d(&tmp, out, 32, 1);
}