package com.lesar.qwiz.api.model.media

import com.google.gson.annotations.SerializedName

data class Media(
	val uri: String,
	@SerializedName("media_type")
	val mediaType: MediaType
)
