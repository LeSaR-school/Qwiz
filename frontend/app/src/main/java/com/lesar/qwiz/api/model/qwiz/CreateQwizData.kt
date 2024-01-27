package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName

data class CreateQwizData(
	@SerializedName("creator_password")
	val creatorPassword: String,
	val qwiz: QwizOnlyData,
	val questions: List<CreateQuestionData>,
)
