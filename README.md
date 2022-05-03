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

Run [bench.mjs](./bench.mjs). There are too many options so the output gets cut-off.
1. unicode generation is the fastest (out of all libraries) - ~3k ops/s
2. svg generation - ~2.7k ops/s
3. png generation - ~1.7k ops/s

*Note*: If you only need to create PNGs, use [magic-qr-code](https://www.npmjs.com/package/magic-qr-code) instead - it uses a faster rust crate.

Svg and Unicode QRCode generations are both faster in this package, but were not benchmarked.