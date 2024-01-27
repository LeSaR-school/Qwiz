package com.lesar.qwiz.viewmodel

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.model.account.AccountType
import com.lesar.qwiz.api.model.assignment.AssignmentData
import com.lesar.qwiz.api.model.assignment.AssignmentRepository
import com.lesar.qwiz.api.model.group.ClassRepository
import kotlinx.coroutines.launch

class ClassViewModel : ViewModel() {

	private val repository: AssignmentRepository = AssignmentRepository()
	private val classRepository: ClassRepository = ClassRepository()

	var assignmentDatas = mutableListOf<AssignmentData>()
	var classId: Int = -1
	lateinit var accountType: AccountType

	private val mGetClassAssignments: MutableLiveData<List<AssignmentData>?> = MutableLiveData()
	val getClassAssignments: LiveData<List<AssignmentData>?>
		get() = mGetClassAssignments

	fun getClassAssignments(accountId: Int, password: String) {
		viewModelScope.launch {
			mGetClassAssignments.value = try {
				val assignments = repository.getAssignments(accountId, password)
				assignments?.filter {
					it.classId == classId && !it.completed
				}
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				null
			}
		}
	}


	private val mDeleteClass: MutableLiveData<Int> = MutableLiveData()
	val deleteClass: LiveData<Int>
		get() = mDeleteClass

	fun deleteClass(password: String) {
		viewModelScope.launch {
			mDeleteClass.value = try {
				classRepository.deleteClass(classId, password).code()
			} catch (e: Exception) {
				Log.d("DEBUG", e.message.toString())
				-1
			}
		}
	}

}