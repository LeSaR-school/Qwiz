package com.lesar.qwiz.api.model.group

import com.google.gson.annotations.SerializedName

data class DeleteClassData(
	@SerializedName("teacher_password")
	val teacherPassword: String,
)
