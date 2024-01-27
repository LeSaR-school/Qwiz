package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.account.Account
import com.lesar.qwiz.api.model.account.AccountRepository
import com.lesar.qwiz.api.model.group.ClassData
import com.lesar.qwiz.api.model.group.ClassRepository
import kotlinx.coroutines.launch

class ProfileViewModel : ViewModel() {

	private val repository: AccountRepository = AccountRepository()
	lateinit var account: Account
	private val classRepository: ClassRepository = ClassRepository()
	var classDatas = mutableListOf<ClassData>()



	private val mGetAccount: MutableLiveData<Account?> = MutableLiveData()
	val getAccount: LiveData<Account?>
		get() = mGetAccount

	fun getAccount(id: Int) {
		viewModelScope.launch {
			mGetAccount.value = try {
				repository.getAccount(id)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

	private val mDeleteAccount: MutableLiveData<Int> = MutableLiveData()
	val deleteAccount: LiveData<Int>
		get() = mDeleteAccount

	fun deleteAccount(id: Int, password: String) {
		viewModelScope.launch {
			mDeleteAccount.value = try {
				repository.deleteAccount(id, password)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

	private val mGetAccountClasses: MutableLiveData<List<ClassData>?> = MutableLiveData()
	val getAccountClasses: LiveData<List<ClassData>?>
		get() = mGetAccountClasses

	fun getAccountClasses(id: Int, password: String) {
		viewModelScope.launch {
			mGetAccountClasses.value = try {
				classRepository.getAccountClasses(id, password)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

}