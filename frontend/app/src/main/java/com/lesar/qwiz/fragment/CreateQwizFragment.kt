package com.lesar.qwiz.fragment

import android.app.Activity
import android.content.Context.MODE_MULTI_PROCESS
import android.content.Intent
import android.os.Bundle
import android.provider.MediaStore
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Toast
import androidx.activity.result.ActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import androidx.navigation.navGraphViewModels
import androidx.recyclerview.widget.LinearLayoutManager
import com.lesar.qwiz.R
import com.lesar.qwiz.api.model.qwiz.CreateQuestionEditData
import com.lesar.qwiz.databinding.FragmentCreateQwizBinding
import com.lesar.qwiz.scroller.QuestionPreviewsAdapter
import com.lesar.qwiz.viewmodel.CreateQwizViewModel

class CreateQwizFragment : Fragment(R.layout.fragment_create_qwiz) {

	private lateinit var binding: FragmentCreateQwizBinding

	private val viewModel: CreateQwizViewModel by navGraphViewModels(R.id.qwiz_create_navigation)



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentCreateQwizBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		viewModel.resultLauncher = registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { handleNewThumbnail(it) }
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		var id: Int? = null
		arguments?.let {
			val args = CreateQwizFragmentArgs.fromBundle(it)
			args.copyEditId.let { copyEditId ->
				id = copyEditId
				viewModel.cloneQwiz(copyEditId)
			}
			viewModel.editing = args.editing
			if (args.editing) {
				binding.bPublish.setText(R.string.qwiz_apply_edits)
			}
		}

		binding.etQwizName.setText(viewModel.name)

		initRecyclerView()
		initClickListeners(id)
		initObservers()
	}

	private fun initRecyclerView() {

		viewModel.adapter?.also {
			it.fragment = this
		} ?: run {
			viewModel.adapter = QuestionPreviewsAdapter(viewModel.questions, this)
		}

		binding.rvQuestionPreviews.adapter = viewModel.adapter
		binding.rvQuestionPreviews.layoutManager = LinearLayoutManager(requireContext())

	}

	private fun initClickListeners(qwizId: Int?) {

		binding.ivEditThumbnail.setOnClickListener {
			try {
				val pickPhoto = Intent(
					Intent.ACTION_PICK,
					MediaStore.Images.Media.EXTERNAL_CONTENT_URI
				)
				viewModel.resultLauncher.launch(pickPhoto)
			} catch (e: Exception) {
				Log.d("DEBUG", "$e")
				Toast.makeText(requireContext(), R.string.internal_error, Toast.LENGTH_SHORT).show()
			}
		}

		binding.bPublish.setOnClickListener {
			val sharedPrefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)
			val id = sharedPrefs.getInt("id", -1)
			val password = sharedPrefs.getString("password", null)
			if (id >= 0 && password != null) {
				if (viewModel.editing) {
					val res = viewModel.editQwiz(binding.etQwizName.text.toString(), qwizId!!, password)
					if (!res) {
						Toast.makeText(
							requireContext(),
							R.string.qwiz_create_fail,
							Toast.LENGTH_LONG
						).show()
						return@setOnClickListener
					}
				} else {
					val res = viewModel.createQwiz(binding.etQwizName.text.toString(), id, password)
					if (!res) {
						Toast.makeText(
							requireContext(),
							R.string.qwiz_create_fail,
							Toast.LENGTH_LONG
						).show()
						return@setOnClickListener
					}
				}
			}
			else {
				Toast.makeText(requireContext(), R.string.login_fail, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			}
			binding.bPublish.isEnabled = false
		}

	}

	private fun initObservers() {

		viewModel.createQwiz.observe(viewLifecycleOwner) {
			when (it) {
				0, 400, 500 -> {
					Log.d("DEBUG", "$it")
					Toast.makeText(requireContext(), R.string.internal_error, Toast.LENGTH_SHORT).show()
				}
				401 -> {
					Log.d("DEBUG", "outdated password")
					Toast.makeText(requireContext(), R.string.login_fail, Toast.LENGTH_SHORT).show()
				}
				201 -> {
					findNavController().popBackStack()
				}
			}
			binding.bPublish.isEnabled = true
		}

		viewModel.editQwiz.observe(viewLifecycleOwner) {
			when (it) {
				0, 400, 500 -> {
					Log.d("DEBUG", "$it")
					Toast.makeText(requireContext(), R.string.internal_error, Toast.LENGTH_SHORT).show()
				}
				401 -> {
					Log.d("DEBUG", "outdated password")
					Toast.makeText(requireContext(), R.string.login_fail, Toast.LENGTH_SHORT).show()
				}
				200 -> {
					findNavController().popBackStack()
				}
			}
			binding.bPublish.isEnabled = true
		}

		viewModel.cloneQwiz.observe(viewLifecycleOwner) {
			if (viewModel.cloned) return@observe

			it?.let { qwiz ->
				viewModel.name = qwiz.name
				binding.etQwizName.setText(qwiz.name)

				for (question in qwiz.questions) {
					question.embed?.let { embed ->
						viewModel.downloadEmbed(question.index, embed.uri)
					}

					val answers = mutableListOf(question.answer1, question.answer2)
					question.answer3?.let { ans -> answers.add(ans) }
					question.answer4?.let { ans -> answers.add(ans) }
					viewModel.questions.add(
						CreateQuestionEditData(
							question.body,
							answers,
							question.correct,
							null
						)
					)
				}

				viewModel.adapter?.notifyItemRangeInserted(0, qwiz.questions.size)

				viewModel.cloned = true
				viewModel.editQuestionStates = MutableList(qwiz.questions.size) { CreateQwizViewModel.QuestionState.None }
			}
		}

		viewModel.downloadEmbed.observe(viewLifecycleOwner) {
			it?.let {
				viewModel.questions[it.questionId].embedBytes = it.data
			}
		}

	}

	private fun handleNewThumbnail(res: ActivityResult) {

		if (res.resultCode != Activity.RESULT_OK) {
			Log.d("DEBUG", "${res.resultCode}")
			return
		}
		res.data?.data?.also { uri ->
			val fileStream = requireActivity().contentResolver.openInputStream(uri)
			fileStream?.also { stream ->
				viewModel.thumbnailBytes = stream.readBytes()
				fileStream.close()

				binding.ivEditThumbnail.setImageURI(uri)
			} ?: run {
				Log.d("DEBUG", "file not found")
			}
		} ?: run {
			Log.d("DEBUG", "no uri selected")
		}

	}

	fun editQuestion(position: Int) {

		if (viewModel.publishing) return

		Log.d("DEBUG", "$position")
		val currentQuestion = viewModel.questions[position]
		val args = QuestionEditFragmentArgs(position, currentQuestion.body, currentQuestion.answers.toTypedArray(), currentQuestion.correct.toInt())
		findNavController().navigate(R.id.action_createQwizFragment_to_questionEditFragment, args.toBundle())

	}

	fun createQuestion() {

		if (viewModel.publishing) return
		if (viewModel.editing) return

		findNavController().navigate(R.id.action_createQwizFragment_to_questionEditFragment)

	}

}