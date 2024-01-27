package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.account.AccountRepository
import com.lesar.qwiz.api.model.account.IdUsernameData
import com.lesar.qwiz.api.model.group.ClassRepository
import kotlinx.coroutines.launch

class CreateClassViewModel : ViewModel() {

	private val accountRepository: AccountRepository = AccountRepository()
	private val repository: ClassRepository = ClassRepository()

	val selectedStudentIdus = mutableSetOf<IdUsernameData>()



	private val mSearchStudents: MutableLiveData<List<IdUsernameData>?> = MutableLiveData()
	val searchStudents: LiveData<List<IdUsernameData>?>
		get() = mSearchStudents

	fun searchStudents(usernamePrefix: String) {

		viewModelScope.launch {
			mSearchStudents.value = try {
				accountRepository.getAccountsWithUsername(usernamePrefix, true)
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				null
			}
		}

	}


	private val mCreateClass: MutableLiveData<Int> = MutableLiveData()
	val createClass: LiveData<Int>
		get() = mCreateClass

	fun createClass(name: String, id: Int, password: String) {

		viewModelScope.launch {
			mCreateClass.value = try {
				repository.createClass(name, id, password, selectedStudentIdus.map { idu -> idu.id }).code()
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				-1
			}
		}

	}



}