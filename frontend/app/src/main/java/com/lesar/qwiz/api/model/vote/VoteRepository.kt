package com.lesar.qwiz.api.model.vote

import com.lesar.qwiz.api.RetrofitProvider
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class VoteRepository {

	suspend fun getVoterIds(qwizId: Int): List<Int>? {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.voteApi.getVoterIds(qwizId)
		}
	}

}