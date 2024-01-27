package com.lesar.qwiz.api.model.group

import com.lesar.qwiz.api.RetrofitProvider
import com.lesar.qwiz.api.model.account.AccountPasswordData
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import retrofit2.Response

class ClassRepository {

	suspend fun getAccountClasses(accountId: Int, password: String): List<ClassData>? {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.classApi.getAccountClasses(accountId, AccountPasswordData(password))
		}
	}

	suspend fun createClass(name: String, teacherId: Int, password: String, studentIds: List<Int>): Response<Void> {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.classApi.createClass(CreateClassData(
				password,
				ClassOnlyData(
					teacherId,
					name,
					studentIds,
				)
			))
		}
	}

	suspend fun deleteClass(classId: Int, password: String): Response<Void> {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.classApi.deleteClass(classId, DeleteClassData(password))
		}
	}

}