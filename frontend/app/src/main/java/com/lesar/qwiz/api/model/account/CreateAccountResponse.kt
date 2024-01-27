package com.lesar.qwiz.api.model.account

data class CreateAccountResponse(
	val code: Int,
	val account: Account?,
	val password: String,
)
