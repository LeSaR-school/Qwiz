package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.assignment.AssignmentRepository
import com.lesar.qwiz.api.model.qwiz.QwizPreview
import com.lesar.qwiz.api.model.qwiz.QwizRepository
import kotlinx.coroutines.launch

class CreateAssignmentViewModel : ViewModel() {

	private val repository: AssignmentRepository = AssignmentRepository()
	private val qwizRepository: QwizRepository = QwizRepository()

	var classId: Int? = null
	var opensAt: Long? = null
	var closesAt: Long? = null

	var selectedQwiz: QwizPreview? = null



	private val mGetAccountQwizes: MutableLiveData<List<QwizPreview>> = MutableLiveData()
	val getAccountQwizes: LiveData<List<QwizPreview>>
		get() = mGetAccountQwizes

	fun getAccountQwizes(id: Int, password: String) {

		viewModelScope.launch {
			mGetAccountQwizes.value = try {
				qwizRepository.getAccountQwizes(id, password)
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				null
			}
		}

	}

	private val mCreateAssignment: MutableLiveData<Int?> = MutableLiveData()
	val createAssignment: LiveData<Int?>
		get() = mCreateAssignment

	fun createAssignment(qwizId: Int, password: String) {

		viewModelScope.launch {
			mCreateAssignment.value = try {
				var res = -1
				classId?.let {
					res = repository.createAssignment(it, qwizId, password, opensAt, closesAt)
				}
				res
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				null
			}
		}

	}

}