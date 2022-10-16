import re
import sympy as sp
from collections import Counter

import plonka


def get_code(n, fn):
	nlen = len('%x' % n)
	x = sp.Matrix([sp.Symbol('s_%0*X' % (nlen, i)) for i in range(n)])
	y = sp.Matrix([sp.Symbol('out[(%*d * stridea) + out_index]' %
				  (len(str(n)), i)) for i in range(n)])
	code = []
	plonka.call_tfm_run(fn, y, x, code)
	outcode = []
	aliases = {}
	for (dst, src) in code:
		dst = str(dst)

		# print(src)

		src = str(src).replace('1.0*', '')
		src = str(src).replace('*1.0', '')

		applied = False
		# a*s_x +- a*s_y -> a * (x +- y)
		m = re.match(
			r'([0-9.-]+)\*(s_[0-9A-F]+) ([+-]) ([0-9.-]+)\*(s_[0-9A-F]+)', src)
		if m:
			cst1, var1, sign, cst2, var2 = m.groups()
			if cst1 == cst2:
				src = '%.6ff32 * (%s %s %s)' % (float(cst1), var1, sign, var2)
				applied = True

		# a*s_x +- b*s_y -> a * s_x +- b * s_y
		if applied == False:
			m = re.match(
				r'([0-9.-]+)\*(s_[0-9A-F]+) ([+-]) ([0-9.-]+)\*(s_[0-9A-F]+)', src)
			if m:
				cst1, var1, sign, cst2, var2 = m.groups()
				src = '%.6ff32 * %s %s %.6ff32 * %s' % (
					float(cst1), var1, sign, float(cst2), var2)
				applied = True

		# a*s_x -> a * s_x
		if applied == False:
			m = re.match(r'([0-9.-]+)\*(s_[0-9A-F]+)', src)
			if m:
				cst1, var1 = m.groups()
				src = '%.6ff32 * %s' % (float(cst1), var1)
				applied = True

		# a*x +- a*y -> a * (x +- y)
		if applied == False:
			m = re.match(
				r'([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x) ([+-]) ([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x)', src)
			if m:
				cst1, var1, sign, cst2, var2 = m.groups()
				if cst1 == cst2:
					src = '%.6ff32 * (%s %s %s)' % (float(cst1),
													var1, sign, var2)
					applied = True

		# a*x +- b*z +- a*y -> a * (x +- y) +- b * z
		if applied == False:
			m = re.match(
				r'([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x) ([+-]) ([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x) ([+-]) ([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x)', src)
			if m:
				cst1, var1, sign2, cst3, var3, sign, cst2, var2 = m.groups()
				if cst1 == cst2:
					src = '%.6ff32 * (%s %s %s) %s %.6ff32 * %s' % (float(cst1),
																	var1, sign, var2, sign2, float(cst3), var3)
					applied = True

		# a*x +- b*y -> a * x +- b * y
		if applied == False:
			m = re.match(
				r'([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x) ([+-]) ([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x)', src)
			if m:
				cst1, var1, sign, cst2, var2 = m.groups()
				src = '%.6ff32 * %s %s %.6ff32 * %s' % (
					float(cst1), var1, sign, float(cst2), var2)
				applied = True

		# a*x -> a * x
		if applied == False:
			m = re.match(r'([0-9.-]+)\*(x[0-9a-f]+_[0-9a-f]+x)', src)
			if m:
				cst1, var1 = m.groups()
				src = '%.6ff32 * %s' % (float(cst1), var1)

		# print('%s\n' % src)

		line = '%s = %s;' % (dst, src)
		if '[' not in dst:
			line = 'let ' + line

		# let a = b;
		if re.match(r'^let x[0-9a-f]+_[0-9a-f]+x = x[0-9a-f]+_[0-9a-f]+x;$', line):
			aliases[dst] = aliases.get(src, src)
			continue

		for var, rep in aliases.items():
			line = line.replace(var, rep)

		line = '\t\t' + line
		outcode.append(line)
	ret = '\n'.join(outcode)

	var_histogram = Counter(re.findall(r'x[0-9a-f]+_[0-9a-f]+x', ret))
	simple_vars = [key for key, val in var_histogram.items() if val == 2]

	for var in simple_vars:
		outcode2 = []
		for line in outcode:
			if ' = %s;' % var in line:
				for line2 in outcode:
					if var in line2:
						data = line2.split('=')[1]
						break
				dst, src = line.split('=')
				line = dst + '=' + data
			outcode2.append(line)
		outcode = outcode2
	ret = '\n'.join(outcode)

	var_histogram = Counter(re.findall(r'x[0-9a-f]+_[0-9a-f]+x', ret))
	orphan_vars = [key for key, val in var_histogram.items() if val == 1]
	for orphan in orphan_vars:
		outcode = [line for line in outcode if orphan not in line]
	ret = '\n'.join(outcode)

	slen = len(str(n))
	srcvars = []
	for i in range(n):
		srcvars.append(
			'\t\tlet s_%0*X = src[(%*d * stridea) + src_index];' % (nlen, i, slen, i))
	ret = '\n'.join(srcvars) + '\n\n' + ret

	varsfrom = sorted(set(re.findall(r'x[0-9a-f]+_[0-9a-f]+x', ret)))
	nb_var = len(varsfrom)
	varsto = ['x_%0*X' % (len('%x' % nb_var), x) for x in range(nb_var)]
	for var_from, var_to in zip(varsfrom, varsto):
		ret = ret.replace(var_from, var_to)

	numbers = re.findall(r'[^x_]([0-9.-]+)f32', ret)
	nlist = sorted(list(set(numbers)))

	maxvarsize = len(str('%x' % len(nlist)))

	nvlist = []
	nvindex = 0
	for num in nlist:
		nvlist.append([num, 'v_%0*X' % (maxvarsize, nvindex)])
		nvindex += 1

	floatmaxsize = 0
	for numvar in nvlist:
		floatlen = len(numvar[0])
		if floatlen > floatmaxsize:
			floatmaxsize = floatlen

	varnumbers = []
	for numvar in nvlist:
		ret = ret.replace(numvar[0] + 'f32', numvar[1])
		varnumbers.append('\tlet %s = %*.6ff32;' %
						  (numvar[1], floatmaxsize, float(numvar[0])))

	if varnumbers != []:
		varnumbersret = '\n'.join(varnumbers)

	fro = re.search(
		r'(;)(\n\t\t)(out\[\([0 ]+ \* stridea\) \+ out_index\])', ret)
	if fro:
		c, ntt, out = fro.groups()
		ret = ret.replace(c + ntt + out, c + '\n' + ntt + out)

	return ret, varnumbersret


def write_dct_code(n, template):
	print('Generating %d' % n)
	print('\tGenerating fast DCT')
	fdct, fvars = get_code(n, 'cosII')
	print('\tGenerating fast IDCT')
	idct, ivars = get_code(n, 'cosIII')
	templatestr = template
	templatestr = templatestr.replace('%BLOCK_SIZE%', str(n))
	templatestr = templatestr.replace('%VARS_FDCT%', fvars)
	templatestr = templatestr.replace('%CODE_FDCT%', fdct)
	templatestr = templatestr.replace('%VARS_IDCT%', ivars)
	templatestr = templatestr.replace('%CODE_IDCT%', idct)
	name = 'dct%d' % n
	open('../src/' + name + '.rs', 'w').write(templatestr)
	return name


if __name__ == '__main__':
	template = open('template.rs').read()
	mod = ''
	for i in range(1, 10):
		mod += 'pub mod ' + write_dct_code(1 << i, template) + ';\n'
	open('../src/lib.rs', 'w').write(mod)
