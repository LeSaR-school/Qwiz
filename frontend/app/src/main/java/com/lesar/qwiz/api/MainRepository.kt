package com.lesar.qwiz.api

import com.lesar.qwiz.api.model.Account
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class MainRepository {

    suspend fun getAccount(id: Int): Account {
        return withContext(Dispatchers.IO) {
            return@withContext RetrofitProvider.qwizApiProvider.getAccount(id)
        }
    }

}