# @khaf/qrcode

A wrapper for the [qrcode](https://crates.io/crates/qrcode/0.12.0) crate.

# API

See the [types](./index.d.ts)

## PNG

```js
import { qrcodeImage } from '@khaf/qrcode'
import { writeFileSync } from 'node:fs'

writeFileSync(
	'./qrcode.png',
	qrcodeImage('text here') // returns a Uint8ClampedArray
)
```

## SVG

*Note*: options are not included in this example.

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
    2 012 ops/s, ±1.65%   | slowest, 32.93% slower

  @khaf/qrcode svg:
    2 850 ops/s, ±0.67%   | 5% slower

  @khaf/qrcode unicode:
    3 000 ops/s, ±0.48%   | fastest

Finished 3 cases!
  Fastest: @khaf/qrcode unicode
  Slowest: @khaf/qrcode image
```

*Note*: If you only need the raw data, use [magic-qr-code](https://www.npmjs.com/package/magic-qr-code) instead.