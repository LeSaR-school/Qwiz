package com.lesar.qwiz.api.model.media

data class DownloadMediaResponse(
	val questionId: Int,
	val data: ByteArray,
)
