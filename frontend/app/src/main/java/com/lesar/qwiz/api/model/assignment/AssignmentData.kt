package com.lesar.qwiz.api.model.assignment

import com.google.gson.annotations.SerializedName

data class AssignmentData(
	val id: Int,
	@SerializedName("qwiz_id")
	val qwizId: Int,
	@SerializedName("qwiz_name")
	val qwizName: String,
	@SerializedName("class_id")
	val classId: Int,
	@SerializedName("open_time")
	val openTime: Long?,
	@SerializedName("close_time")
	val closeTime: Long?,
	val completed: Boolean,
)
