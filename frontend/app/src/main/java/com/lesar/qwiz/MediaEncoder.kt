package com.lesar.qwiz

import android.util.Base64

object MediaEncoder {
	fun encode(data: ByteArray): String {
		return Base64.encodeToString(data, Base64.URL_SAFE or Base64.NO_PADDING or Base64.NO_WRAP)
	}
}