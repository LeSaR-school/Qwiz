package com.lesar.qwiz.api.model.qwiz

data class CreateQuestionEditData(
	val body: String,
	val answers: MutableList<String>,
	val correct: Short,
	var embedBytes: ByteArray?,
)
