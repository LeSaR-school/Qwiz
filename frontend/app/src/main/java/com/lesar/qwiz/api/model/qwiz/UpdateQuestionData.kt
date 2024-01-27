package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.CreateMediaData

data class UpdateQuestionData(
	@SerializedName("creator_password")
	val creatorPassword: String,
	@SerializedName("new_body")
	val newBody: String,
	@SerializedName("new_answers")
	val newAnswers: List<NewAnswer>?,
	@SerializedName("new_correct")
	val newCorrect: Short,
	@SerializedName("new_embed")
	val newEmbed: CreateMediaData?,
)
