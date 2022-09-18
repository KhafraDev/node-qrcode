import b from 'benny'

import { qrcodeImage, qrcodeSvg, qrcodeUnicode } from './index.js'

await b.suite(
	'Generate QR codes',
	b.add('@khaf/qrcode image', () => {
		qrcodeImage('Hello, world!')
	}),
	b.add('@khaf/qrcode svg', () => {
		qrcodeSvg('Hello, world!')
	}),
	b.add('@khaf/qrcode unicode', () => {
		qrcodeUnicode('Hello, world!')
	}),
	b.cycle(),
	b.complete()
)