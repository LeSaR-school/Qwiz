package com.lesar.qwiz.api

import com.lesar.qwiz.api.model.account.Account
import com.lesar.qwiz.api.model.account.AccountPasswordData
import com.lesar.qwiz.api.model.account.CreateAccountData
import com.lesar.qwiz.api.model.account.IdUsernameData
import com.lesar.qwiz.api.model.account.UpdateAccountData
import com.lesar.qwiz.api.model.account.VerifyAccountPasswordByUsernameData
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.HTTP
import retrofit2.http.PATCH
import retrofit2.http.POST
import retrofit2.http.Path
import retrofit2.http.Query

interface AccountApi {

	@GET("account/{id}")
	suspend fun getAccount(@Path(value = "id") id: Int): Account
	@GET("account/search?<username_prefix>&<is_student>")
	suspend fun getAccountsWithUsername(@Query("username_prefix") usernamePrefix: String, @Query("is_student") isStudent: Boolean?): List<IdUsernameData>?

	@POST("account/{id}/verify")
	suspend fun verifyAccountPasswordById(@Path(value = "id") id: Int, @Body body: AccountPasswordData): Response<Account?>

	@POST("account/verify")
	suspend fun verifyAccountPasswordByUsername(@Body body: VerifyAccountPasswordByUsernameData): Response<Account?>

	@POST("account")
	suspend fun createAccount(@Body body: CreateAccountData): Response<Account?>

	@PATCH("account/{id}")
	suspend fun updateAccount(@Path(value = "id") id: Int, @Body body: UpdateAccountData): Response<Void>

	@HTTP(method = "DELETE", path = "account/{id}", hasBody = true)
	suspend fun deleteAccount(@Path(value = "id") id: Int, @Body body: AccountPasswordData): Response<Void>

}