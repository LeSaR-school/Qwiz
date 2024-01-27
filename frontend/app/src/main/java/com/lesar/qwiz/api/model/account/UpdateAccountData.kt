package com.lesar.qwiz.api.model.account

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.CreateMediaData

data class UpdateAccountData (
	val password: String,
	@SerializedName("new_username")
	val newUsername: String?,
	@SerializedName("new_password")
	val newPassword: String?,
	@SerializedName("new_account_type")
	val newAccountType: AccountType?,
	@SerializedName("new_profile_picture")
	val newProfilePicture: CreateMediaData?,
)
