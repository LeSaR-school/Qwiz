package com.lesar.qwiz.api.model

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lesar.qwiz.api.MainRepository
import kotlinx.coroutines.launch

class MainViewModel : ViewModel() {

    private val repository: MainRepository = MainRepository()

    private val mData: MutableLiveData<Account> = MutableLiveData()
    val data: LiveData<Account>
        get() = mData

    fun getAccount(id: Int) {
        viewModelScope.launch {
            val result = repository.getAccount(id)
            mData.value = result
        }
    }

}