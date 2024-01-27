package com.lesar.qwiz.viewmodel

import android.content.Intent
import android.util.Log
import androidx.activity.result.ActivityResultLauncher
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.BASE_URL
import com.lesar.qwiz.api.model.media.DownloadMediaResponse
import com.lesar.qwiz.api.model.qwiz.CreateQuestionEditData
import com.lesar.qwiz.api.model.qwiz.Qwiz
import com.lesar.qwiz.api.model.qwiz.QwizRepository
import com.lesar.qwiz.scroller.QuestionPreviewsAdapter
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import java.net.URL

class CreateQwizViewModel : ViewModel() {

	enum class QuestionState {
		None,
		Edited,
	}

	private val repository: QwizRepository = QwizRepository()

	lateinit var resultLauncher: ActivityResultLauncher<Intent>

	var adapter: QuestionPreviewsAdapter? = null

	var name: String = ""
	var questions: MutableList<CreateQuestionEditData> = mutableListOf()
	var thumbnailBytes: ByteArray? = null
	var embedBytes: ByteArray? = null

	var currentlyEditing: Int? = null

	var publishing: Boolean = false
		private set
	var editing: Boolean = false

	var editQuestionStates: MutableList<QuestionState>? = null



	fun addQuestion(body: String, answers: MutableList<String>, correct: Short, embedBytes: ByteArray?) {
		questions.add(
			CreateQuestionEditData(body, answers, correct, embedBytes)
		)
		adapter?.let {
			it.notifyItemInserted(it.itemCount - 1)
		}
	}

	fun updateQuestion(body: String, answers: MutableList<String>, correct: Short, embedBytes: ByteArray?) {
		currentlyEditing?.let {
			questions[it] = CreateQuestionEditData(body, answers, correct, embedBytes)
			adapter?.notifyItemChanged(it)
			currentlyEditing = null

			editQuestionStates?.let { states ->
				states[it] = QuestionState.Edited
			}
		}
	}

	fun deleteQuestion() {
		if (editing) return

		currentlyEditing?.let {
			questions.removeAt(it)
			adapter?.notifyItemRemoved(it)
			currentlyEditing = null
		}
	}



	private val mCreateQwiz: MutableLiveData<Int> = MutableLiveData()
	val createQwiz: LiveData<Int>
		get() = mCreateQwiz

	fun createQwiz(name: String, id: Int, password: String): Boolean {

		if (name.isEmpty()) return false
		if (questions.isEmpty()) return false

		publishing = true

		viewModelScope.launch {
			mCreateQwiz.value = try {
				val res = repository.createQwiz(id, password, name, thumbnailBytes, questions)
				res.code()
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				0
			}
			publishing = false
		}

		return true

	}



	private val mEditQwiz: MutableLiveData<Int> = MutableLiveData()
	val editQwiz: LiveData<Int>
		get() = mEditQwiz

	fun editQwiz(name: String, id: Int, password: String): Boolean {

		if (name.isEmpty()) return false
		if (questions.isEmpty()) return false

		publishing = true

		viewModelScope.launch {
			mEditQwiz.value = try {
				for ((index, question) in questions.withIndex().filter { editQuestionStates!![it.index] == QuestionState.Edited }) {
					Log.d("DEBUG", "${repository.updateQuestion(id, password, index, question)}")
				}
				val res = repository.editQwiz(id, password, name, thumbnailBytes)
				Log.d("DEBUG", "${res.body()}")
				res.code()
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				0
			}
			publishing = false
		}

		return true

	}

	var cloned: Boolean = false

	private val mCloneQwiz: MutableLiveData<Qwiz?> = MutableLiveData()
	val cloneQwiz: LiveData<Qwiz?>
		get() = mCloneQwiz

	fun cloneQwiz(id: Int) {

		viewModelScope.launch {
			mCloneQwiz.value = try {
				repository.getQwiz(id)
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				null
			}
		}

	}

	private val mDownloadEmbed: MutableLiveData<DownloadMediaResponse?> = MutableLiveData()
	val downloadEmbed: LiveData<DownloadMediaResponse?>
		get() = mDownloadEmbed

	fun downloadEmbed(questionId: Int, urlString: String) {

		viewModelScope.launch(Dispatchers.IO) {
			val res = try {
				val url = URL("$BASE_URL$urlString")
				val res = url.readBytes()
				DownloadMediaResponse(questionId, res)
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				null
			}
			mDownloadEmbed.postValue(res)
		}

	}

}