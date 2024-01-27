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
import com.lesar.qwiz.databinding.FragmentPlainBodyBinding
import com.lesar.qwiz.viewmodel.QuestionViewModel
import com.lesar.qwiz.viewmodel.QwizViewModel

class PlainBodyFragment : Fragment(R.layout.fragment_plain_body) {

	private lateinit var binding: FragmentPlainBodyBinding

	private val viewModel: QwizViewModel by navGraphViewModels(R.id.qwiz_navigation)
	private val questionViewModel: QuestionViewModel by viewModels({ requireParentFragment() })



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentPlainBodyBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		val question = viewModel.qwiz.questions.getOrNull(questionViewModel.currentQuestion)
		question?.also {
			binding.tvQuestionBody.text = it.body
		} ?: run {
			Log.d("DEBUG", "out of questions")
		}
	}

}