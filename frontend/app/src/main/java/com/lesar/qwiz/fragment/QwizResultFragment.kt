package com.lesar.qwiz.fragment

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.View.VISIBLE
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import com.lesar.qwiz.R
import com.lesar.qwiz.databinding.FragmentQwizResultBinding

class QwizResultFragment : Fragment(R.layout.fragment_qwiz_result) {

	private lateinit var binding: FragmentQwizResultBinding



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentQwizResultBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		arguments?.let {
			val args = QwizResultFragmentArgs.fromBundle(it)
			binding.tvScore.text = getString(R.string.out_of, args.correct.toString(), args.total.toString())
			if (args.finishedAssignment) binding.tvAssignmentComplete.visibility = VISIBLE
		} ?: run {
			findNavController().popBackStack()
		}

		initClickListeners()
	}

	private fun initClickListeners() {
		binding.bOk.setOnClickListener {
			findNavController().popBackStack()
		}
	}

}