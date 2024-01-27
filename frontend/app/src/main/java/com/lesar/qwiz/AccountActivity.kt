package com.lesar.qwiz

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import com.lesar.qwiz.databinding.ActivityAccountBinding
import com.lesar.qwiz.fragment.ProfileFragment

class AccountActivity : AppCompatActivity() {

	private lateinit var binding: ActivityAccountBinding

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		binding = ActivityAccountBinding.inflate(layoutInflater)
		setContentView(binding.root)
	}
}