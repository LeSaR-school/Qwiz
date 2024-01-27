package com.lesar.qwiz.api.model.qwiz

import com.lesar.qwiz.api.model.media.Media

data class Question(
	val index: Int,
	val body: String,
	val answer1: String,
	val answer2: String,
	val answer3: String?,
	val answer4: String?,
	val embed: Media?,
	val correct: Short,
)
