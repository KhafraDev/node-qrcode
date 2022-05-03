import test from 'ava'

import { qrcodeUnicode } from '../index.js'
import { join } from 'path'
import { fileURLToPath } from 'url'
import { readFileSync } from 'fs'

const fixtures = join(fileURLToPath(import.meta.url), '..', 'fixtures')

const invalidTypes = [
	null,
	undefined,
	true,
	false,
	0,
	{ a: 'b' },
	new Date()
]

test('qrcodeUnicode', (t) => {
	const qr = qrcodeUnicode('hello world')

	t.assert(typeof qr === 'string')
	t.is(qr, readFileSync(join(fixtures, 'unicode.txt'), 'utf8'))
})

test('qrcodeUnicode - Bad arguments', (t) => {
	// ensure process doesn't crash
	t.throws(() => qrcodeUnicode())
	
	for (const arg of invalidTypes) {
	  t.throws(() => qrcodeUnicode(arg))
	}
  
	t.throws(() => qrcodeUnicode(''), {
	  code: 'GenericFailure',
	  message: 'text length must be greater than 0'
	})
})