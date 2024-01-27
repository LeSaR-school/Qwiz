package com.lesar.qwiz.api

import retrofit2.http.GET
import retrofit2.http.Path

interface VoteApi {

	@GET("vote/{qwiz_id}")
	suspend fun getVoterIds(@Path(value = "qwiz_id") qwizId: Int): List<Int>?

}