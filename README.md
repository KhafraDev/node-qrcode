# @khaf/qrcode

Native bindings to create QR-codes in a variety of formats!

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
    6 618 ops/s, ±0.14%   | fastest

  @khaf/qrcode svg:
    2 785 ops/s, ±1.46%   | slowest, 57.92% slower

  @khaf/qrcode unicode:
    2 919 ops/s, ±2.76%   | 55.89% slower

Finished 3 cases!
  Fastest: @khaf/qrcode image
  Slowest: @khaf/qrcode svg
```

*Note*: If you only need the raw data, use [magic-qr-code](https://www.npmjs.com/package/magic-qr-code) instead.