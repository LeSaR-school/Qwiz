package com.lesar.qwiz.api.model.qwiz

import com.lesar.qwiz.api.model.media.CreateMediaData

data class CreateQuestionData(
	val body: String,
	val answer1: String,
	val answer2: String,
	val answer3: String?,
	val answer4: String?,
	val correct: Short,
	val embed: CreateMediaData?,
)
