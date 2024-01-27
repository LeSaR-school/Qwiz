package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.CreateMediaData

data class AddQuestionData(
	@SerializedName("creator_password")
	val creatorPassword: String,
	val question: CreateQuestionData
)
