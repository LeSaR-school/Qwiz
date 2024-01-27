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
import com.lesar.qwiz.databinding.Fragment2AnswersBinding
import com.lesar.qwiz.viewmodel.QuestionViewModel
import com.lesar.qwiz.viewmodel.QwizViewModel

class Answers2Fragment : Fragment(R.layout.fragment_2_answers) {

	private lateinit var binding: Fragment2AnswersBinding
	
	private val viewModel: QwizViewModel by navGraphViewModels(R.id.qwiz_navigation)
	private val questionViewModel: QuestionViewModel by viewModels({ requireParentFragment() })



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = Fragment2AnswersBinding.inflate(inflater, container, false)
		return binding.root
	}
	
	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		val question = viewModel.qwiz.questions.getOrNull(questionViewModel.currentQuestion)
		question?.also {
			binding.tvAnswer1.text = it.answer1
			binding.tvAnswer2.text = it.answer2
		} ?: run {
			Log.d("DEBUG", "out of questions")
		}

		initClickListeners()

	}

	private fun initClickListeners() {

		binding.tvAnswer1.setOnClickListener {
			nextQuestion(1)
		}
		binding.tvAnswer2.setOnClickListener {
			nextQuestion(2)
		}

	}

	private fun nextQuestion(answer: Short) {
		val questionFragment = parentFragment as QuestionFragment
		questionFragment.nextQuestion(answer)
		disableAnswers()
	}

	private fun disableAnswers() {
		binding.tvAnswer1.isClickable = false
		binding.tvAnswer2.isClickable = false
	}

}