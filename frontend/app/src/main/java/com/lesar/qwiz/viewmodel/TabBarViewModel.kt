package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.account.AccountRepository
import com.lesar.qwiz.api.model.account.VerifyAccountPasswordResponse
import kotlinx.coroutines.launch

class TabBarViewModel : ViewModel() {

	private val repository: AccountRepository = AccountRepository()



	private val mVerifyPassword: MutableLiveData<VerifyAccountPasswordResponse?> = MutableLiveData()
	val verifyPassword: LiveData<VerifyAccountPasswordResponse?>
		get() = mVerifyPassword

	fun verifyPassword(id: Int, password: String) {
		viewModelScope.launch {
			mVerifyPassword.value = try {
				repository.verifyPassword(id, password)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

}