package com.lesar.qwiz.api.model.account

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.CreateMediaData

data class CreateAccountData(
	val username: String,
	val password: String,
	@SerializedName("account_type")
	val accountType: AccountType,
	@SerializedName("profile_picture")
	val profilePicture: CreateMediaData?,
)
