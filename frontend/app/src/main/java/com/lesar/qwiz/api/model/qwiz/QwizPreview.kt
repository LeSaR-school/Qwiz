package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName

data class QwizPreview(
	val id: Int,
	val name: String,
	@SerializedName("creator_name")
	val creatorName: String,
	@SerializedName("create_time")
	val createTime: Long,
	val votes: Int,
	@SerializedName("thumbnail_uri")
	val thumbnailUri: String?,
	@SerializedName("creator_profile_picture_uri")
	val creatorProfilePictureUri: String?,
)