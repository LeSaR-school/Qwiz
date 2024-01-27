package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName

data class SolveQwizResponse(
	val correct: UInt,
	val total: UInt,
	val results: List<Boolean>,
	@SerializedName("assignment_complete")
	val assignmentComplete: Boolean?,
)
