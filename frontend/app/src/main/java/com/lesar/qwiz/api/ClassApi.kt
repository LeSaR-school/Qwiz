package com.lesar.qwiz.api

import com.lesar.qwiz.api.model.account.AccountPasswordData
import com.lesar.qwiz.api.model.group.AddStudentsData
import com.lesar.qwiz.api.model.group.ClassData
import com.lesar.qwiz.api.model.group.CreateClassData
import com.lesar.qwiz.api.model.group.DeleteClassData
import com.lesar.qwiz.api.model.group.RemoveStudentsData
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.HTTP
import retrofit2.http.POST
import retrofit2.http.PUT
import retrofit2.http.Path

interface ClassApi {

	@POST("account/{id}/classes")
	suspend fun getAccountClasses(@Path(value = "id") id: Int, @Body body: AccountPasswordData): List<ClassData>?

	@POST("class")
	suspend fun createClass(@Body body: CreateClassData): Response<Void>

	@PUT("class/{id}")
	suspend fun addStudents(@Path(value = "id") id: Int, @Body body: AddStudentsData): Response<Void>

	@HTTP(method = "DELETE", path = "class/{id}", hasBody = true)
	suspend fun deleteClass(@Path(value = "id") id: Int, @Body body: DeleteClassData): Response<Void>

	@HTTP(method = "DELETE", path = "class/{id}", hasBody = true)
	suspend fun removeStudents(@Path(value = "id") id: Int, @Body body: RemoveStudentsData): Response<Void>

}