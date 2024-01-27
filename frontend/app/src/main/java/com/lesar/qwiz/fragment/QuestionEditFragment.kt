package com.lesar.qwiz.fragment

import android.app.AlertDialog
import android.content.Intent
import android.graphics.BitmapFactory
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
import com.lesar.qwiz.R
import com.lesar.qwiz.databinding.FragmentQuestionEditBinding
import com.lesar.qwiz.viewmodel.CreateQwizViewModel

class QuestionEditFragment : Fragment(R.layout.fragment_question_edit) {

	private lateinit var binding: FragmentQuestionEditBinding

	private val viewModel: CreateQwizViewModel by navGraphViewModels(R.id.qwiz_create_navigation)



	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		viewModel.resultLauncher = registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { handleNewEmbed(it) }
	}

	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentQuestionEditBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		arguments?.also {
			val args = QuestionEditFragmentArgs.fromBundle(it)
			initQuestionState(args)
		} ?: run {
			findNavController().popBackStack()
		}

		initClickListeners()
	}

	private fun initQuestionState(args: QuestionEditFragmentArgs) {
		binding.etBody.setText(args.body)
		binding.etAnswer1.setText(args.answers?.get(0) ?: "")
		binding.etAnswer2.setText(args.answers?.get(1) ?: "")
		args.answers?.getOrNull(2)?.let { answer3 ->
			binding.etAnswer3.isEnabled = true
			binding.etAnswer3.setText(answer3)
			binding.swEnable3.isChecked = true
			binding.rbCorrect3.isEnabled = true
			binding.swEnable4.isEnabled = true
		}
		args.answers?.getOrNull(3)?.let { answer4 ->
			binding.etAnswer4.isEnabled = true
			binding.etAnswer4.setText(answer4)
			binding.swEnable4.isChecked = true
			binding.rbCorrect4.isEnabled = true
		}
		when (args.correct) {
			1 -> binding.rbCorrect1
			2 -> binding.rbCorrect2
			3 -> binding.rbCorrect3
			4 -> binding.rbCorrect4
			else -> null
		}?.isChecked = true

		if (args.position >= 0) {
			viewModel.currentlyEditing = args.position
			val question = viewModel.questions[args.position]
			question.embedBytes?.let { bytes ->
				val bmp = BitmapFactory.decodeByteArray(bytes, 0, bytes.size)
				binding.ivEmbed.setImageBitmap(bmp)
			}
			if (!viewModel.editing) binding.bDeleteQuestion.isEnabled = true
		} else {
			viewModel.currentlyEditing = null
		}
	}

	private fun initClickListeners() {

		binding.bSaveQuestion.setOnClickListener {

			if (binding.etBody.text.isEmpty()) {
				Toast.makeText(context, R.string.qwiz_fill_in_question, Toast.LENGTH_SHORT).show()
				binding.etBody.requestFocus()
				return@setOnClickListener
			}

			if (binding.etAnswer1.text.isEmpty()) {
				Toast.makeText(context, R.string.qwiz_fill_answer_1, Toast.LENGTH_SHORT).show()
				binding.etAnswer1.requestFocus()
				return@setOnClickListener
			}
			if (binding.etAnswer2.text.isEmpty()) {
				Toast.makeText(context, R.string.qwiz_fill_answer_2, Toast.LENGTH_SHORT).show()
				binding.etAnswer2.requestFocus()
				return@setOnClickListener
			}
			if (binding.swEnable3.isChecked && binding.swEnable3.isEnabled && binding.etAnswer3.text.isEmpty()) {
				Toast.makeText(context, R.string.qwiz_fill_answer_3, Toast.LENGTH_SHORT).show()
				binding.etAnswer3.requestFocus()
				return@setOnClickListener
			}
			if (binding.swEnable4.isChecked && binding.swEnable4.isEnabled && binding.etAnswer4.text.isEmpty()) {
				Toast.makeText(context, R.string.qwiz_fill_answer_4, Toast.LENGTH_SHORT).show()
				binding.etAnswer4.requestFocus()
				return@setOnClickListener
			}

			if (binding.rgCorrect.checkedRadioButtonId < 0) {
				Toast.makeText(context, R.string.qwiz_select_correct, Toast.LENGTH_SHORT).show()
				binding.rgCorrect.requestFocus()
				return@setOnClickListener
			}

			val answers = mutableListOf(binding.etAnswer1.text.toString(), binding.etAnswer2.text.toString())
			if (binding.swEnable3.isChecked && binding.swEnable3.isEnabled && binding.etAnswer3.text.isNotEmpty()) answers.add(binding.etAnswer3.text.toString())
			if (binding.swEnable4.isChecked && binding.swEnable4.isEnabled && binding.etAnswer4.text.isNotEmpty()) answers.add(binding.etAnswer4.text.toString())

			val correct: Short = when (binding.rgCorrect.checkedRadioButtonId) {
				R.id.rbCorrect1 -> 1
				R.id.rbCorrect2 -> 2
				R.id.rbCorrect3 -> 3
				R.id.rbCorrect4 -> 4
				else -> 0
			}

			if (viewModel.currentlyEditing != null) {
				viewModel.updateQuestion(binding.etBody.text.toString(), answers, correct, viewModel.embedBytes)
			} else {
				viewModel.addQuestion(binding.etBody.text.toString(), answers, correct, viewModel.embedBytes)
			}
			findNavController().popBackStack()

		}

		binding.bDeleteQuestion.setOnClickListener {
			AlertDialog.Builder(context)
				.setTitle(R.string.delete_question)
				.setMessage(R.string.delete_question_confirm)
				.setPositiveButton(R.string.yes) { _, _ ->
					viewModel.deleteQuestion()
					findNavController().popBackStack()
				}
				.setNegativeButton(R.string.no, null).show()
		}

		binding.swEnable3.setOnCheckedChangeListener { _, checked ->
			if (checked) {
				binding.etAnswer3.isEnabled = true
				binding.rbCorrect3.isEnabled = true
				binding.swEnable4.isEnabled = true
			} else {
				binding.etAnswer3.isEnabled = false
				binding.rbCorrect3.isEnabled = false
				if (binding.rbCorrect3.isChecked || binding.rbCorrect4.isChecked) {
					binding.rgCorrect.clearCheck()
				}
				binding.swEnable4.isEnabled = false
			}
		}

		binding.swEnable4.setOnCheckedChangeListener { _, checked ->
			if (checked) {
				binding.etAnswer4.isEnabled = true
				binding.rbCorrect4.isEnabled = true
			} else {
				binding.etAnswer4.isEnabled = false
				binding.rbCorrect4.isEnabled = false
				if (binding.rbCorrect4.isChecked) {
					binding.rgCorrect.clearCheck()
				}
			}
		}

		binding.ivEmbed.setOnClickListener {
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

	}

	private fun handleNewEmbed(res: ActivityResult) {

		res.data?.data?.also { uri ->
			val fileStream = requireActivity().contentResolver.openInputStream(uri)
			fileStream?.also { stream ->
				viewModel.embedBytes = stream.readBytes()
				fileStream.close()

				binding.ivEmbed.setImageURI(uri)
			} ?: run {
				Log.d("DEBUG", "file not found")
			}
		} ?: run {
			Log.d("DEBUG", "no uri selected")
		}

	}

}