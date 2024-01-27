package com.lesar.qwiz.api

import com.lesar.qwiz.api.model.account.AccountPasswordData
import com.lesar.qwiz.api.model.qwiz.CreateQwizData
import com.lesar.qwiz.api.model.qwiz.DeleteQwizData
import com.lesar.qwiz.api.model.qwiz.EditQwizData
import com.lesar.qwiz.api.model.qwiz.Qwiz
import com.lesar.qwiz.api.model.qwiz.QwizPreview
import com.lesar.qwiz.api.model.qwiz.SolveQwizData
import com.lesar.qwiz.api.model.qwiz.SolveQwizResponse
import com.lesar.qwiz.api.model.qwiz.UpdateQuestionData
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.HTTP
import retrofit2.http.PATCH
import retrofit2.http.POST
import retrofit2.http.Path
import retrofit2.http.Query

interface QwizApi {

	@GET("qwiz/{id}")
	suspend fun getQwiz(@Path(value = "id") id: Int): Qwiz?

	@POST("account/{id}/qwizes")
	suspend fun getAccountQwizes(@Path(value = "id") id: Int, @Body body: AccountPasswordData): List<QwizPreview>

	@GET("qwiz/best")
	suspend fun getBestQwizPreviews(@Query("page") page: Int = 0, @Query("search") search: String? = null): List<QwizPreview>

	@POST("qwiz")
	suspend fun createQwiz(@Body body: CreateQwizData): Response<Void>

	@GET("qwiz/recent")
	suspend fun getRecentQwizPreviews(@Query("page") page: Int = 0, @Query("search") search: String? = null): List<QwizPreview>

	@POST("qwiz/{id}/solve")
	suspend fun solveQwiz(@Path(value = "id") id: Int, @Body body: SolveQwizData, @Query("assignment_id") assignmentId: Int?): SolveQwizResponse?

	@HTTP(method = "DELETE", path = "qwiz/{id}", hasBody = true)
	suspend fun deleteQwiz(@Path(value = "id") id: Int, @Body body: DeleteQwizData): Response<Void>

	@PATCH("qwiz/{id}")
	suspend fun editQwiz(@Path(value = "id") qwizId: Int, @Body body: EditQwizData): Response<Void>

	@PATCH("question/{qwiz_id}/{index}")
	suspend fun updateQuestion(@Path(value = "qwiz_id") qwizId: Int, @Path(value = "index") questionIndex: Int, @Body body: UpdateQuestionData): Response<Void>

}