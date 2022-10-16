// GENERATED CODE
// Fast DCT IDCT %BLOCK_SIZE%x%BLOCK_SIZE%

#![allow(non_snake_case)]

#[inline(always)]
fn fast_dct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
%VARS_FDCT%

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..%BLOCK_SIZE% {
%CODE_FDCT%

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_dct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; %BLOCK_SIZE% * %BLOCK_SIZE%];
	fast_dct_1d(src, &mut tmp, 1, %BLOCK_SIZE%);
	fast_dct_1d(&tmp, out, %BLOCK_SIZE%, 1);
}

#[inline(always)]
fn fast_idct_1d(src: &[f32], out: &mut [f32], stridea: usize, strideb: usize) {
%VARS_IDCT%

	let mut src_index = 0;
	let mut out_index = 0;

	for _ in 0..%BLOCK_SIZE% {
%CODE_IDCT%

		src_index += strideb;
		out_index += strideb;
	}
}

pub fn fast_idct(src: &[f32], out: &mut [f32]) {
	let mut tmp = vec![0.0f32; %BLOCK_SIZE% * %BLOCK_SIZE%];
	fast_idct_1d(src, &mut tmp, 1, %BLOCK_SIZE%);
	fast_idct_1d(&tmp, out, %BLOCK_SIZE%, 1);
}