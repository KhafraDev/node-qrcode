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

test('qrcodeSvg - Only color options', (t) => {
  const qr = qrcodeSvg('hello world', {
    darkColor: '#ffff00',
    lightColor: '#ff44ff'
  })

  t.assert(typeof qr === 'string')
  t.is(qr, readFileSync(join(fixtures, 'color_options.svg'), 'utf8'))
})

test('qrcodeSvg - Only dimensions', (t) => {
  const qr = qrcodeSvg('hello world', {
    minHeight: 420,
    minWidth: 420
  })

  t.assert(typeof qr === 'string')
  t.is(qr, readFileSync(join(fixtures, 'min_dimensions.svg'), 'utf8'))
})

test('qrcodeSvg - No options', (t) => {
  t.is(
    qrcodeSvg('hello world'),
    readFileSync(join(fixtures, 'no_options.svg'), 'utf8')
  )
})

test('qrcodeSvg - Empty options', (t) => {
  t.is(
    qrcodeSvg('hello world', {}),
    readFileSync(join(fixtures, 'empty_options.svg'), 'utf8')
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