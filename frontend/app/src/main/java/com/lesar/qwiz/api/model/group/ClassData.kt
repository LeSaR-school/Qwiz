package com.lesar.qwiz.api.model.group

import com.google.gson.annotations.SerializedName

data class ClassData(
	val id: Int,
	val name: String,
	@SerializedName("teacher_id")
	val teacherId: Int,
	@SerializedName("teacher_name")
	val teacherName: String,
	@SerializedName("student_ids")
	val studentIds: List<Int>,
)
