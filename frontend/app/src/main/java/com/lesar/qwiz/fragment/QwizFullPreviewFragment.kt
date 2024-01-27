package com.lesar.qwiz.fragment

import android.content.Context.MODE_MULTI_PROCESS
import android.icu.text.SimpleDateFormat
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.View.GONE
import android.view.View.VISIBLE
import android.view.ViewGroup
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import androidx.navigation.navGraphViewModels
import com.lesar.qwiz.R
import com.lesar.qwiz.api.BASE_URL
import com.lesar.qwiz.databinding.FragmentQwizFullPreviewBinding
import com.lesar.qwiz.viewmodel.QwizViewModel
import com.squareup.picasso.Picasso
import java.util.Date

class QwizFullPreviewFragment : Fragment(R.layout.fragment_qwiz_full_preview) {

	private lateinit var binding: FragmentQwizFullPreviewBinding

	private val viewModel: QwizViewModel by navGraphViewModels(R.id.qwiz_navigation)



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentQwizFullPreviewBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		if (viewModel.assignmentComplete) {
			findNavController().popBackStack()
			return
		}

		arguments?.also {
			val args = QwizFullPreviewFragmentArgs.fromBundle(it)
			viewModel.assignmentId = args.assignmentId
			if (args.assignmentId >= 0) {
				binding.bCopyQwiz.visibility = GONE
				binding.bEditQwiz.visibility = GONE
				binding.bDeleteQwiz.visibility = GONE
			}
			viewModel.getQwiz(args.qwizId)
		} ?: run {
			Toast.makeText(context, R.string.qwiz_not_found, Toast.LENGTH_SHORT).show()
			findNavController().popBackStack()
			return
		}

		initClickListeners()
		initObservers()

	}

	private fun initClickListeners() {

		binding.bTakeQwiz.setOnClickListener {
			findNavController().navigate(R.id.action_qwizFullPreviewFragment_to_plainQuestionFragment)
		}

		binding.bDeleteQwiz.setOnClickListener {
			val sharedPrefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)
			sharedPrefs.getString("password", null)?.let {
				viewModel.deleteQwiz(it)
				binding.bDeleteQwiz.isEnabled = false
			}
		}

		binding.bEditQwiz.setOnClickListener {
			val args = CreateQwizFragmentArgs(viewModel.qwiz.id, true)
			findNavController().navigate(R.id.action_qwizFullPreviewFragment_to_qwiz_create_navigation, args.toBundle())
		}

		binding.bCopyQwiz.setOnClickListener {
			val args = CreateQwizFragmentArgs(viewModel.qwiz.id, false)
			findNavController().navigate(R.id.action_qwizFullPreviewFragment_to_qwiz_create_navigation, args.toBundle())
		}

	}

	private fun initObservers() {

		viewModel.getQwiz.observe(viewLifecycleOwner) {
			it?.also { qwiz ->
				binding.bTakeQwiz.isEnabled = true
				binding.bCopyQwiz.isEnabled = true

				viewModel.qwiz = qwiz

				binding.tvQwizName.text = qwiz.name

				val dt = Date(qwiz.createTime)
				val formattedTime = SimpleDateFormat("dd/MM/yy", resources.configuration.locales.get(0)).format(dt)
				binding.tvCreateTime.text = formattedTime

				qwiz.thumbnail?.let { thumbnail ->
					Picasso.get()
						.load("$BASE_URL${thumbnail.uri}")
						.into(binding.ivQwizThumbnail)
				}
				binding.tvVotesNumber.text = qwiz.votes.toString()

				viewModel.getAccount(qwiz.creatorID)

				val sharedPrefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)
				if (qwiz.creatorID == sharedPrefs.getInt("id", -1) && viewModel.assignmentId < 0) {
					binding.bDeleteQwiz.visibility = VISIBLE
					binding.bDeleteQwiz.isEnabled = true
					binding.bEditQwiz.visibility = VISIBLE
					binding.bEditQwiz.isEnabled = true
				}
			} ?: run {
				Toast.makeText(context, R.string.load_qwiz_fail, Toast.LENGTH_LONG).show()
				findNavController().popBackStack()
			}
		}

		viewModel.getAccount.observe(viewLifecycleOwner) {
			it?.also {
				binding.tvQwizCreator.text = it.username
			} ?: run {
				Toast.makeText(context, R.string.load_qwiz_creator_fail, Toast.LENGTH_LONG).show()
			}
		}

		viewModel.deleteQwiz.observe(viewLifecycleOwner) {
			it?.let { res ->
				when (res.status) {
					200 -> {
						Toast.makeText(context, R.string.delete_qwiz_success, Toast.LENGTH_SHORT).show()
						findNavController().popBackStack()
					}
					401 -> Toast.makeText(context, R.string.login_fail, Toast.LENGTH_LONG).show()
					0, 500 -> Toast.makeText(context, R.string.internal_error, Toast.LENGTH_LONG).show()
					else -> Toast.makeText(context, R.string.delete_qwiz_fail, Toast.LENGTH_LONG).show()
				}
			}

			binding.bDeleteQwiz.isEnabled = true
		}

	}

}