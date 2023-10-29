package com.lesar.qwiz.api.model

data class Account(
    val id: Int,
    val username: String,
    val profile_picture: Media,
    val account_type: AccountType,
)
