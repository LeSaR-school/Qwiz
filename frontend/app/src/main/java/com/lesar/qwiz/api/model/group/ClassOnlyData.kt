package com.lesar.qwiz.api.model.group

import com.google.gson.annotations.SerializedName

data class ClassOnlyData(
	@SerializedName("teacher_id")
	val teacherId: Int,
	val name: String,
	@SerializedName("student_ids")
	val studentIds: List<Int>?,
)