package com.lesar.qwiz

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.lesar.qwiz.databinding.ActivityMainBinding




class MainActivity : AppCompatActivity() {

	private lateinit var binding: ActivityMainBinding

	override fun onCreate(savedInstanceState: Bundle?) {
//		val appLocale = LocaleListCompat.forLanguageTags("ru-RU")
//		AppCompatDelegate.setApplicationLocales(appLocale)

		super.onCreate(savedInstanceState)

		binding = ActivityMainBinding.inflate(layoutInflater)
		setContentView(binding.root)
	}

}