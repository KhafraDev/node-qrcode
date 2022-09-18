# @khaf/qrcode

Native bindings to create QR-codes in a variety of formats!

# API

See the [types](./index.d.ts)

## PNG

*Note*: `qrcodeImage` returns a Uint8ClampedArray!

```js
import { qrcodeImage } from '@khaf/qrcode'
import { writeFileSync } from 'node:fs'

writeFileSync('./qrcode.png', qrcodeImage('text here'))
```

## SVG

```js
import { qrcodeSvg } from '@khaf/qrcode'

const svg = qrcodeSvg('text here')
// do something with svg
```

## Unicode

```js
import { qrcodeUnicode } from '@khaf/qrcode'

console.log(qrcodeUnicode('text here'))
```




# Benchmarks

```
Running "Generate QR codes" suite...
Progress: 100%

  @khaf/qrcode image:
    6 606 ops/s, ±0.91%    | slowest, 52.8% slower

  @khaf/qrcode svg:
    11 969 ops/s, ±0.90%   | 14.48% slower

  @khaf/qrcode unicode:
    13 995 ops/s, ±1.54%   | fastest

Finished 3 cases!
  Fastest: @khaf/qrcode unicode
  Slowest: @khaf/qrcode image
```

*Note*: If you only need the raw data, use [magic-qr-code](https://www.npmjs.com/package/magic-qr-code) instead.