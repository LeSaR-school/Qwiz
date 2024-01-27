package com.lesar.qwiz.api.model.account

import android.util.Log
import com.lesar.qwiz.api.RetrofitProvider
import com.lesar.qwiz.api.model.media.CreateMediaData
import com.lesar.qwiz.api.model.media.MediaType
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class AccountRepository {

	suspend fun getAccount(id: Int): Account {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.accountApi.getAccount(id)
		}
	}

	suspend fun getAccountsWithUsername(usernamePrefix: String, isStudent: Boolean? = null): List<IdUsernameData>? {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.accountApi.getAccountsWithUsername(usernamePrefix, isStudent)
		}
	}

	suspend fun verifyPassword(id: Int, password: String): VerifyAccountPasswordResponse {
		return withContext(Dispatchers.IO) {
			val res = RetrofitProvider.accountApi.verifyAccountPasswordById(
				id,
				AccountPasswordData(password)
			)
			return@withContext VerifyAccountPasswordResponse(res.code(), res.body(), password)
		}
	}

	suspend fun verifyPassword(username: String, password: String): VerifyAccountPasswordResponse {
		return withContext(Dispatchers.IO) {
			val res = RetrofitProvider.accountApi.verifyAccountPasswordByUsername(
				VerifyAccountPasswordByUsernameData(username, password)
			)
			return@withContext VerifyAccountPasswordResponse(res.code(), res.body(), password)
		}
	}

	suspend fun createAccount(username: String, password: String, accountType: AccountType): CreateAccountResponse {
		return withContext(Dispatchers.IO) {
			val res = RetrofitProvider.accountApi.createAccount(
				CreateAccountData(username, password, accountType, null)
			)
			return@withContext CreateAccountResponse(res.code(), res.body(), password)
		}
	}

	suspend fun updateAccount(id: Int, password: String, newUsername: String?, newPassword: String?, newAccountType: AccountType?, newProfilePictureData: String?): UpdateAccountResponse {
		return withContext(Dispatchers.IO) {
			val newProfilePicture = newProfilePictureData?.let { CreateMediaData(it, MediaType.Image) }
			val res = RetrofitProvider.accountApi.updateAccount(
				id,
				UpdateAccountData(password, newUsername, newPassword, newAccountType, newProfilePicture)
			)
			Log.d("DEBUG", "${res.body()}")
			return@withContext UpdateAccountResponse(res.code(), newUsername, newPassword, newAccountType, newProfilePicture)
		}
	}

	suspend fun deleteAccount(id: Int, password: String): Int {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.accountApi.deleteAccount(
				id,
				AccountPasswordData(password)
			).code()
		}
	}

}