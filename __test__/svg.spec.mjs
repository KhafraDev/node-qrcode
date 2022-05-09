import test from 'ava'

import { qrcodeSvg } from '../index.js'

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
    qrcodeSvg('hello world')
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