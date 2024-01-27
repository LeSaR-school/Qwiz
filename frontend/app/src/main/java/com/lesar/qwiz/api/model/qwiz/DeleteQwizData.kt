package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName

data class DeleteQwizData(
	@SerializedName("creator_password")
	val creatorPassword: String,
)