package com.lesar.qwiz.api

import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory

object RetrofitProvider {

    private fun getProvider(): Retrofit {

        return Retrofit.Builder()
            .baseUrl(URL)
            .addConverterFactory(GsonConverterFactory.create())
            .build()

    }

    val qwizApiProvider: QwizApi = getProvider().create(QwizApi::class.java)

}