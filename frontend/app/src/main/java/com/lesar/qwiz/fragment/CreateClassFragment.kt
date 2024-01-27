package com.lesar.qwiz.fragment

import android.app.AlertDialog
import android.content.Context
import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import com.lesar.qwiz.R
import com.lesar.qwiz.databinding.FragmentCreateClassBinding
import com.lesar.qwiz.viewmodel.CreateClassViewModel


class CreateClassFragment : Fragment(R.layout.fragment_create_class) {

	private lateinit var binding: FragmentCreateClassBinding
	
	private val viewModel: CreateClassViewModel by viewModels()



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentCreateClassBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		initClickListeners()
		initObservers()

	}

	private fun initClickListeners() {

		binding.bSearchStudents.setOnClickListener {

			if (binding.etSearchStudents.text.isEmpty()) {
				Toast.makeText(context, "Enter a student to search for", Toast.LENGTH_LONG).show()
				return@setOnClickListener
			}

			val usernamePrefix = binding.etSearchStudents.text.toString()
			viewModel.searchStudents(usernamePrefix)
			binding.bSearchStudents.isEnabled = false

		}

		binding.bCreateClass.setOnClickListener {

			val sharedPrefs = requireContext().getSharedPreferences("user", Context.MODE_MULTI_PROCESS)
			val id = sharedPrefs.getInt("id", -1)
			val password = sharedPrefs.getString("password", "")
			if (id < 0 || password == null) {
				Toast.makeText(context, R.string.internal_error, Toast.LENGTH_SHORT).show()
				findNavController().popBackStack()
				return@setOnClickListener
			}
			if (binding.etClassName.text.isEmpty()) {
				Toast.makeText(context, R.string.class_name, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			}
			binding.bCreateClass.isEnabled = false
			viewModel.createClass(binding.etClassName.text.toString(), id, password)

		}

	}

	private fun initObservers() {

		viewModel.searchStudents.observe(viewLifecycleOwner) { idus ->
			binding.bSearchStudents.isEnabled = true
			idus?.also {
				if (idus.isEmpty()) {
					Toast.makeText(context, R.string.search_no_students, Toast.LENGTH_LONG).show()
					return@also
				}
				val usernames = it.map { idu -> idu.username }
				val selected = it.map { idu -> viewModel.selectedStudentIdus.contains(idu) }.toMutableList()
				val builder: AlertDialog.Builder = AlertDialog.Builder(context)
				builder
					.setTitle(R.string.select_students)
					.setPositiveButton(R.string.ok) { _, _ ->
						for ((idu, isSelected) in idus.zip(selected)) {
							if (isSelected) {
								viewModel.selectedStudentIdus.add(idu)
							} else {
								viewModel.selectedStudentIdus.remove(idu)
							}
						}
						updateSelectedList()
					}
					.setNegativeButton(R.string.cancel) {_,_->}
					.setMultiChoiceItems(usernames.toTypedArray(), selected.toBooleanArray()) { _, index, isChecked ->
						Log.d("DEBUG", "$index is checked: $isChecked")
						selected[index] = isChecked
					}

				builder.create().show()
			} ?: run {
				Toast.makeText(context, R.string.internal_error, Toast.LENGTH_SHORT).show()
			}
		}

		viewModel.createClass.observe(viewLifecycleOwner) { status ->
			when (status) {
				201 -> {
					findNavController().popBackStack()
				}
				else -> {
					Log.d("DEBUG", "$status")
					Toast.makeText(context, R.string.internal_error, Toast.LENGTH_SHORT).show()
				}
			}
		}

	}

	private fun updateSelectedList() {
		val students = viewModel.selectedStudentIdus.joinToString("\n") { idu -> idu.username }
		binding.tvSelectedStudents.text = students
	}

}