package com.lesar.qwiz.api.model.group

import com.google.gson.annotations.SerializedName

data class AddStudentsData(
	@SerializedName("teacher_password")
	val teacherPassword: String,
	@SerializedName("student_ids")
	val studentIds: List<Int>,
)
