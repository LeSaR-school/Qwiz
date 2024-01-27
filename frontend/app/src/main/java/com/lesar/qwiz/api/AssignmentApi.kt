package com.lesar.qwiz.api

import com.lesar.qwiz.api.model.account.AccountPasswordData
import com.lesar.qwiz.api.model.assignment.AssignmentData
import com.lesar.qwiz.api.model.assignment.CreateAssignmentData
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.POST
import retrofit2.http.Path

interface AssignmentApi {

	@POST("account/{id}/assignments")
	suspend fun getAccountAssignments(@Path(value = "id") id: Int, @Body body: AccountPasswordData): List<AssignmentData>?

	@POST("class/{id}/assignments")
	suspend fun createAssignment(@Path(value = "id") id: Int, @Body body: CreateAssignmentData): Response<Void>

}