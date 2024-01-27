package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.qwiz.QwizRepository
import com.lesar.qwiz.api.model.qwiz.SolveQwizResponse
import kotlinx.coroutines.launch

class QuestionViewModel : ViewModel() {

	private val repository: QwizRepository = QwizRepository()
	var currentQuestion: Int = 0
	lateinit var currentAnswers: MutableList<Short>

	private var mSolveQwiz: MutableLiveData<SolveQwizResponse?> = MutableLiveData()
	val solveQwiz: LiveData<SolveQwizResponse?>
		get() = mSolveQwiz
	fun solveQwiz(username: String?, qwizId: Int, assignmentId: Int) {
		currentQuestion = 0
		viewModelScope.launch {
			mSolveQwiz.value = try {
				repository.solveQwiz(qwizId, username, currentAnswers, assignmentId)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

}