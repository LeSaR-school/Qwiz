package com.lesar.qwiz.api

import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory

object RetrofitProvider {

	private fun getProvider(): Retrofit {

		return Retrofit.Builder()
			.baseUrl(BASE_URL)
			.addConverterFactory(GsonConverterFactory.create())
			.build()

	}

	val accountApi: AccountApi = getProvider().create(AccountApi::class.java)
	val qwizApi: QwizApi = getProvider().create(QwizApi::class.java)
	val voteApi: VoteApi = getProvider().create(VoteApi::class.java)
	val classApi: ClassApi = getProvider().create(ClassApi::class.java)
	val assignmentApi: AssignmentApi = getProvider().create(AssignmentApi::class.java)

}