package com.lesar.qwiz.fragment

import android.content.Context.MODE_MULTI_PROCESS
import android.os.Bundle
import android.view.View
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import androidx.navigation.navGraphViewModels
import com.lesar.qwiz.R
import com.lesar.qwiz.viewmodel.QuestionViewModel
import com.lesar.qwiz.viewmodel.QwizViewModel

class QuestionFragment : Fragment(R.layout.fragment_question) {

	private val qwizViewModel: QwizViewModel by navGraphViewModels(R.id.qwiz_navigation)
	private val viewModel: QuestionViewModel by viewModels()



	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		viewModel.currentAnswers = MutableList(qwizViewModel.qwiz.questions.size) {-1}

		updateFragments()
		initObservers()

	}

	private fun initObservers() {
		viewModel.solveQwiz.observe(viewLifecycleOwner) {
			it?.also { res ->
				if (res.assignmentComplete == true) {
					qwizViewModel.assignmentComplete = true
				}
				val args = QwizResultFragmentArgs(res.correct.toInt(), res.total.toInt(), res.assignmentComplete == true)
				findNavController().navigate(R.id.action_plainQuestionFragment_to_qwizResultFragment, args.toBundle())
			} ?: run {
				Toast.makeText(requireContext(), R.string.solve_qwiz_fail, Toast.LENGTH_LONG).show()
			}
		}
	}

	fun nextQuestion(answer: Short) {
		viewModel.currentAnswers[qwizViewModel.qwiz.questions[viewModel.currentQuestion].index] = answer
		viewModel.currentQuestion += 1
		if (viewModel.currentQuestion >= qwizViewModel.qwiz.questions.size) {
			finishQwiz()
		} else {
			updateFragments()
		}
	}

	private fun updateFragments() {
		val question = qwizViewModel.qwiz.questions[viewModel.currentQuestion]
		val bodyFragment = if (question.embed == null) {
			PlainBodyFragment()
		} else {
			ImageBodyFragment()
		}

		val answersFragment = if (question.answer3 == null) {
			Answers2Fragment()
		} else if (question.answer4 == null) {
			Answers3Fragment()
		} else {
			Answers4Fragment()
		}

		childFragmentManager
			.beginTransaction()
			.replace(R.id.fBody, bodyFragment)
			.replace(R.id.fAnswers, answersFragment)
			.commit()
	}

	private fun finishQwiz() {
		val sharedPrefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)
		val username = sharedPrefs.getString("username", null)
		viewModel.solveQwiz(username, qwizViewModel.qwiz.id, qwizViewModel.assignmentId)
	}

}