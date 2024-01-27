package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.account.Account
import com.lesar.qwiz.api.model.account.AccountRepository
import com.lesar.qwiz.api.model.qwiz.DeleteQwizResponse
import com.lesar.qwiz.api.model.qwiz.Qwiz
import com.lesar.qwiz.api.model.qwiz.QwizRepository
import com.lesar.qwiz.api.model.vote.VoteRepository
import kotlinx.coroutines.launch

class QwizViewModel : ViewModel() {

	private val repository: QwizRepository = QwizRepository()
	private val voteRepository: VoteRepository = VoteRepository()
	private val accountRepository: AccountRepository = AccountRepository()

	lateinit var qwiz: Qwiz
	var assignmentId: Int = -1
	var assignmentComplete = false



	private val mGetQwiz: MutableLiveData<Qwiz?> = MutableLiveData()
	val getQwiz: LiveData<Qwiz?>
		get() = mGetQwiz
	fun getQwiz(id: Int) {
		viewModelScope.launch {
			mGetQwiz.value = try {
				repository.getQwiz(id)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

	private val mGetAccount: MutableLiveData<Account?> = MutableLiveData()
	val getAccount: LiveData<Account?>
		get() = mGetAccount
	fun getAccount(id: Int) {
		viewModelScope.launch {
			mGetAccount.value = try {
				accountRepository.getAccount(id)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}


	private var mDeleteQwiz: MutableLiveData<DeleteQwizResponse?> = MutableLiveData()
	val deleteQwiz: LiveData<DeleteQwizResponse?>
		get() = mDeleteQwiz
	fun deleteQwiz(password: String) {
		viewModelScope.launch {
			mDeleteQwiz.value = try {
				val res = repository.deleteQwiz(qwiz.id, password)
				DeleteQwizResponse(res.code(), qwiz.id)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

}