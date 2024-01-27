package com.lesar.qwiz.fragment

import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.navGraphViewModels
import com.lesar.qwiz.R
import com.lesar.qwiz.api.BASE_URL
import com.lesar.qwiz.databinding.FragmentImageBodyBinding
import com.lesar.qwiz.viewmodel.QuestionViewModel
import com.lesar.qwiz.viewmodel.QwizViewModel
import com.squareup.picasso.Picasso

class ImageBodyFragment : Fragment(R.layout.fragment_image_body) {

	private lateinit var binding: FragmentImageBodyBinding

	private val viewModel: QwizViewModel by navGraphViewModels(R.id.qwiz_navigation)
	private val questionViewModel: QuestionViewModel by viewModels({ requireParentFragment() })



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentImageBodyBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		val question = viewModel.qwiz.questions.getOrNull(questionViewModel.currentQuestion)
		question?.also {
			binding.tvQuestionBody.text = it.body
			Picasso.get()
				.load("$BASE_URL${it.embed!!.uri}")
				.into(binding.ivImageEmbed)
		} ?: run {
			Log.d("DEBUG", "out of questions")
		}

	}

}