package com.lesar.qwiz.api

import com.lesar.qwiz.api.model.*
import retrofit2.http.GET
import retrofit2.http.Path

interface QwizApi {

    @GET("account/{id}")
    suspend fun getAccount(@Path(value = "id") id: Int): Account

}