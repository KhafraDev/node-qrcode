import test from 'ava'

import { qrcodeSvg } from '../index.js'
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

test('qrcodeSvg', (t) => {
  t.is(
    qrcodeSvg('hello world'),
    readFileSync(join(fixtures, 'default.svg'), 'utf8')
  )
})

test('qrcodeSvg - Bad arguments', (t) => {
  // ensure process doesn't crash
  t.throws(() => qrcodeSvg())
  
  for (const arg of invalidTypes) {
    t.throws(() => qrcodeSvg(arg))
  }

  t.throws(() => qrcodeSvg(''), {
    code: 'GenericFailure',
    message: 'text length must be greater than 0'
  })
})