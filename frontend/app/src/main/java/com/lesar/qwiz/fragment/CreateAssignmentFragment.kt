package com.lesar.qwiz.fragment

import android.app.AlertDialog
import android.app.DatePickerDialog
import android.app.TimePickerDialog
import android.content.Context
import android.icu.text.SimpleDateFormat
import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import com.lesar.qwiz.R
import com.lesar.qwiz.api.BASE_URL
import com.lesar.qwiz.api.model.qwiz.QwizPreview
import com.lesar.qwiz.databinding.FragmentCreateAssignmentBinding
import com.lesar.qwiz.viewmodel.CreateAssignmentViewModel
import com.squareup.picasso.Picasso
import java.util.Calendar
import java.util.Date


class CreateAssignmentFragment : Fragment(R.layout.fragment_create_qwiz) {

	private lateinit var binding: FragmentCreateAssignmentBinding
	private lateinit var qwizPreview: View

	private val viewModel: CreateAssignmentViewModel by viewModels()



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentCreateAssignmentBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		arguments?.also {
			val args = CreateAssignmentFragmentArgs.fromBundle(it)
			viewModel.classId = args.classId
		} ?: {
			Toast.makeText(context, R.string.internal_error, Toast.LENGTH_SHORT).show()
			findNavController().popBackStack()
		}

		val layoutInflater: LayoutInflater = LayoutInflater.from(requireContext())
		qwizPreview = layoutInflater.inflate(R.layout.view_qwiz_preview, null)
		qwizPreview.findViewById<TextView>(R.id.tvQwizPreviewName).setText(R.string.select_qwiz)
		loadPreview()

		binding.flQwiz.addView(qwizPreview)

		initClickListeners()
		initObservers()

	}

	private fun initClickListeners() {

		qwizPreview.setOnClickListener {

			val sharedPrefs = requireActivity().getSharedPreferences("user", Context.MODE_MULTI_PROCESS)
			val id = sharedPrefs.getInt("id", -1)
			val password = sharedPrefs.getString("password", null)
			if (id < 0 || password == null) {
				Toast.makeText(context, R.string.internal_error, Toast.LENGTH_SHORT).show()
				findNavController().popBackStack()
				return@setOnClickListener
			}

			viewModel.getAccountQwizes(id, password)

		}

		binding.cbOpensAt.setOnCheckedChangeListener { _, checked ->
			if (!checked) {
				viewModel.opensAt = null
				return@setOnCheckedChangeListener
			}

			pickDateTime(true)
		}
		binding.cbClosesAt.setOnCheckedChangeListener { _, checked ->
			if (!checked) {
				viewModel.closesAt = null
				return@setOnCheckedChangeListener
			}

			pickDateTime(false)
		}

		binding.bAssign.setOnClickListener {

			if (viewModel.selectedQwiz == null) {
				Toast.makeText(context, R.string.select_qwiz, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			}
			val sharedPrefs = requireActivity().getSharedPreferences("user", Context.MODE_MULTI_PROCESS)
			val password = sharedPrefs.getString("password", null)
			password?.let {
				binding.bAssign.isEnabled = false
				viewModel.createAssignment(viewModel.selectedQwiz!!.id, password)
			}

		}

	}

	private fun initObservers() {

		viewModel.getAccountQwizes.observe(viewLifecycleOwner) { qwizPreviews ->
			var selected: QwizPreview? = null
			val qwizNames = qwizPreviews.map { preview -> preview.name }
			val builder: AlertDialog.Builder = AlertDialog.Builder(context)
			builder
				.setTitle(R.string.select_qwiz)
				.setPositiveButton(R.string.ok) { _, _ ->
					selected?.let {
						viewModel.selectedQwiz = it
						loadPreview()
					}
				}
				.setNegativeButton(R.string.cancel) {_,_->}
				.setSingleChoiceItems(qwizNames.toTypedArray(), -1) { _, selectedIndex -> selected = qwizPreviews[selectedIndex] }

			builder.create().show()
		}

		viewModel.createAssignment.observe(viewLifecycleOwner) { status ->
			Log.d("DEBUG", "$status")
			when (status) {
				201 -> {
					findNavController().popBackStack()
				}
				else -> {
					Toast.makeText(context, R.string.login_fail, Toast.LENGTH_LONG).show()
					binding.bAssign.isEnabled = true
				}
			}
		}

	}

	private fun pickDateTime(forOpenTime: Boolean) {

		val date: Calendar = Calendar.getInstance()
		val timeDialog = TimePickerDialog(requireContext(), { _, hour, minute ->
			date.set(Calendar.HOUR_OF_DAY, hour)
			date.set(Calendar.MINUTE, minute)
			date.set(Calendar.SECOND, 0)
			val pattern = "HH:mm dd/MM/yy"
			if (forOpenTime) {
				viewModel.opensAt = date.timeInMillis / 1000
				binding.tvCreateOpenTime.text = SimpleDateFormat(pattern, resources.configuration.locales.get(0)).format(date.time)
			} else {
				viewModel.closesAt = date.timeInMillis / 1000
				binding.tvCreateCloseTime.text = SimpleDateFormat(pattern, resources.configuration.locales.get(0)).format(date.time)
			}
		}, 0, 0, true)
		timeDialog.setOnCancelListener {
			if (forOpenTime) {
				binding.cbOpensAt
			} else {
				binding.cbClosesAt
			}.isChecked = false
		}

		val dateDialog = DatePickerDialog(requireContext(), { _, year, month, day ->
			date.set(year, month, day)
			timeDialog.show()
		}, date.get(Calendar.YEAR), date.get(Calendar.MONTH), date.get(Calendar.DAY_OF_MONTH))
		dateDialog.setOnCancelListener {
			if (forOpenTime) {
				binding.cbOpensAt
			} else {
				binding.cbClosesAt
			}.isChecked = false
		}
		dateDialog.show()

	}

	private fun loadPreview() {
		viewModel.selectedQwiz?.let {
			qwizPreview.apply {
				findViewById<TextView>(R.id.tvQwizPreviewName).text = it.name
				findViewById<TextView>(R.id.tvQwizPreviewCreator).text = it.creatorName
				findViewById<TextView>(R.id.tvPreviewVotesNumber).text = it.votes.toString()

				val dt = Date(it.createTime)
				val formattedTime = SimpleDateFormat("dd/MM/yy", resources.configuration.locales.get(0)).format(dt)
				findViewById<TextView>(R.id.tvCreateTime).text = formattedTime

				it.thumbnailUri?.let {
					Picasso.get()
						.load("$BASE_URL$it")
						.into(findViewById<ImageView>(R.id.ivQwizPreviewThumbnail))
				}
			}
		}
	}

}