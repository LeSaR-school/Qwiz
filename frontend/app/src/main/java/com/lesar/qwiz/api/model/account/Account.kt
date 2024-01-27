package com.lesar.qwiz.api.model.account

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.Media

data class Account(
	val id: Int,
	val username: String,
	@SerializedName("profile_picture")
	val profilePicture: Media?,
	@SerializedName("account_type")
	val accountType: AccountType,
)
