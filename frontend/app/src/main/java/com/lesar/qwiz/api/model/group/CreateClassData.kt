package com.lesar.qwiz.api.model.group

import com.google.gson.annotations.SerializedName


data class CreateClassData(
	@SerializedName("teacher_password")
	val teacherPassword: String,
	@SerializedName("class")
	val classData: ClassOnlyData,
)
