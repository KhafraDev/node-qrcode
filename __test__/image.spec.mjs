import test from 'ava'

import { qrcodeImage } from '../index.js'
import { readFileSync } from 'fs'
import { join } from 'path'
import { fileURLToPath } from 'url'

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

test('qrcodeImage', (t) => {
  const qr = qrcodeImage('Hello world')

  t.assert(qr instanceof Uint8ClampedArray)
  t.deepEqual(qr, qrcodeImage('Hello world'))
  t.notDeepEqual(qr, qrcodeImage('Goodbye Earth!'))
  t.deepEqual(Buffer.from(qr), readFileSync(join(fixtures, 'hello_world.png')))
})

test('qrcodeImage - Bad arguments', (t) => {
	// ensure process doesn't crash
	t.throws(() => qrcodeImage())
  
	for (const arg of invalidTypes) {
	  t.throws(() => qrcodeImage(arg))
	}

  t.throws(() => qrcodeImage(''), {
    code: 'GenericFailure',
    message: 'text length must be greater than 0'
  })
})