package com.lesar.qwiz.api.model.assignment

import android.util.Log
import com.lesar.qwiz.api.RetrofitProvider
import com.lesar.qwiz.api.model.account.AccountPasswordData
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class AssignmentRepository {

	suspend fun getAssignments(id: Int, password: String): List<AssignmentData>? {
		Log.d("DEBUG", "$id $password")
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.assignmentApi.getAccountAssignments(id, AccountPasswordData(password))
		}
	}

	suspend fun createAssignment(classId: Int, qwizId: Int, password: String, openTime: Long?, closeTime: Long?): Int {
		return withContext(Dispatchers.IO) {
			val res = RetrofitProvider.assignmentApi.createAssignment(
				classId,
				CreateAssignmentData(password, qwizId, openTime, closeTime)
			)
			Log.d("DEBUG", "$res")
			return@withContext res.code()
		}
	}

}