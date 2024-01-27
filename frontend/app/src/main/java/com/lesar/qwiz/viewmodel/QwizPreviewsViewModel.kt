package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.qwiz.QwizPreview
import com.lesar.qwiz.api.model.qwiz.QwizRepository
import kotlinx.coroutines.launch

class QwizPreviewsViewModel : ViewModel() {

	private val repository: QwizRepository = QwizRepository()
	var qwizPreviews = mutableListOf<QwizPreview>()

	enum class SortBy {
		Votes,
		Recent,
	}

	var sortBy: SortBy = SortBy.Votes

	private val mGetQwizPreviews: MutableLiveData<List<QwizPreview>?> = MutableLiveData()
	val getQwizPreviews: LiveData<List<QwizPreview>?>
		get() = mGetQwizPreviews

	fun getBestQwizPreviews(page: Int = 0, search: String? = null) {
		viewModelScope.launch {
			mGetQwizPreviews.value = try {
				repository.getBestQwizPreviews(page, search)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

	fun getRecentQwizPreviews(page: Int = 0, search: String? = null) {
		viewModelScope.launch {
			mGetQwizPreviews.value = try {
				repository.getRecentQwizPreviews(page, search)
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}

}