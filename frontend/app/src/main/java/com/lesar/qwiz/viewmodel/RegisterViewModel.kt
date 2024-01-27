package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.account.AccountRepository
import com.lesar.qwiz.api.model.account.AccountType
import com.lesar.qwiz.api.model.account.CreateAccountResponse
import kotlinx.coroutines.launch

class RegisterViewModel : ViewModel() {

	private val repository: AccountRepository = AccountRepository()

	private val mCreateAccount: MutableLiveData<CreateAccountResponse?> = MutableLiveData()
	val createAccount: LiveData<CreateAccountResponse?>
		get() = mCreateAccount

	fun createAccount(username: String, password: String, accountType: AccountType) {
		viewModelScope.launch {
			mCreateAccount.value = try {
				repository.createAccount(username, password, accountType)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

}