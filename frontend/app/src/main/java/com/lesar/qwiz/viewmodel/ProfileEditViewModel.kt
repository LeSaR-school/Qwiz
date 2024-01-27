package com.lesar.qwiz.viewmodel

import android.content.Intent
import android.util.Log
import androidx.activity.result.ActivityResultLauncher
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.account.AccountRepository
import com.lesar.qwiz.api.model.account.AccountType
import com.lesar.qwiz.api.model.account.UpdateAccountResponse
import com.lesar.qwiz.fragment.ProfileEditFragmentArgs
import kotlinx.coroutines.launch

class ProfileEditViewModel : ViewModel() {

	private val repository: AccountRepository = AccountRepository()

	lateinit var args: ProfileEditFragmentArgs

	lateinit var resultLauncher: ActivityResultLauncher<Intent>
	var newProfilePictureBytes: ByteArray? = null

	private val mUpdateAccount: MutableLiveData<UpdateAccountResponse?> = MutableLiveData()
	val updateAccount: LiveData<UpdateAccountResponse?>
		get() = mUpdateAccount

	fun updateAccount(id: Int, password: String, newUsername: String?, newPassword: String?, newAccountType: AccountType?, newProfilePictureBase64: String?) {
		viewModelScope.launch {
			mUpdateAccount.value = try {
				repository.updateAccount(id, password, newUsername, newPassword, newAccountType, newProfilePictureBase64)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}
}