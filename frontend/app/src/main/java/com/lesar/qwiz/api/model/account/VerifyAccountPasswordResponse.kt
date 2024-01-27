package com.lesar.qwiz.api.model.account

data class VerifyAccountPasswordResponse(
	val code: Int,
	val account: Account?,
	val password: String,
)