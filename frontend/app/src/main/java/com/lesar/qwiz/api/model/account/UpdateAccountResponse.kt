package com.lesar.qwiz.api.model.account

import com.lesar.qwiz.api.model.media.CreateMediaData

data class UpdateAccountResponse(
	val code: Int,
	val newUsername: String?,
	val newPassword: String?,
	val newAccountType: AccountType?,
	val newProfilePicture: CreateMediaData?,
)
