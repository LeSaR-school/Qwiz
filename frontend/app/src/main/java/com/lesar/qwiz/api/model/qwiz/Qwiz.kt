package com.lesar.qwiz.api.model.qwiz

import com.google.gson.annotations.SerializedName
import com.lesar.qwiz.api.model.media.Media

data class Qwiz(
	val id: Int,
	val name: String,
	@SerializedName("creator_id")
	val creatorID: Int,
	val thumbnail: Media?,
	val questions: List<Question>,
	val public: Boolean,
	@SerializedName("create_time")
	val createTime: Long,
	val votes: UInt,
)
