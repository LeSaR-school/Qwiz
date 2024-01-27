package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.CreateMediaData

data class QwizOnlyData(
	val name: String,
	@SerializedName("creator_id")
	val creatorId: Int,
	val thumbnail: CreateMediaData?,
	val public: Boolean?
)
