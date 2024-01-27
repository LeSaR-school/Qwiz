package com.lesar.qwiz.api.model.assignment

import com.google.gson.annotations.SerializedName

data class CreateAssignmentData(
	@SerializedName("teacher_password")
	val teacherPassword: String,
	@SerializedName("qwiz_id")
	val qwizId: Int,
	@SerializedName("open_time")
	val openTime: Long?,
	@SerializedName("close_time")
	val closeTime: Long?,
)
