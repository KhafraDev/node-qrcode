import b from 'benny'

import { qrcodeImage, qrcodeSvg, qrcodeUnicode } from './index.js'
import { AwesomeQR } from '@satont/awesome-qr'
import QRCode from 'qrcode'
import qrcodeGenerator from 'qrcode-generator'
import AsyncQrCode from '@primecode/async-qrcode'

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
	b.add('@satont/awesome-qr', async () => {
		await new AwesomeQR({
			text: 'Hello, world!',
			size: 200
		}).draw();
	}),
	b.add('qrcode', async () => {
		await QRCode.toBuffer('Hello, world')
	}),
	b.add('qrcode-generator', () => {
		const generator = qrcodeGenerator(4, 'L')
		generator.addData('Hello, world!')
		generator.make()

		generator.createASCII()
	}),
	b.add('@primecode/async-qrcode', async () => {
		await AsyncQrCode.QrCode.generate({
			data: "another string to transform",
			type: AsyncQrCode.Types.SQUARE,
		})
	}),
	b.cycle(),
	b.complete()
)