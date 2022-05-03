import test from 'ava'

import { qrcodeUnicode } from '../index.js'

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