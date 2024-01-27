package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.CreateMediaData

data class EditQwizData(
	@SerializedName("creator_password")
	val creatorPassword: String,
	@SerializedName("new_name")
	val newName: String,
	@SerializedName("new_thumbnail")
	val newThumbnail: CreateMediaData?,
)
