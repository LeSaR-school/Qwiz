package com.lesar.qwiz.fragment

import android.app.AlertDialog
import android.content.Context.MODE_MULTI_PROCESS
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.View.VISIBLE
import android.view.ViewGroup
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import androidx.recyclerview.widget.LinearLayoutManager
import com.lesar.qwiz.R
import com.lesar.qwiz.api.model.account.AccountType
import com.lesar.qwiz.databinding.FragmentClassBinding
import com.lesar.qwiz.scroller.AssignmentsAdapter
import com.lesar.qwiz.viewmodel.ClassViewModel

class ClassFragment : Fragment(R.layout.fragment_class) {

	private lateinit var binding: FragmentClassBinding

	private val viewModel: ClassViewModel by viewModels()

	private lateinit var adapter: AssignmentsAdapter

	private lateinit var sharedPrefs: SharedPreferences



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentClassBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		arguments?.also {
			val args = ClassFragmentArgs.fromBundle(it)
			viewModel.classId = args.classId
			binding.tvClassName.text = args.className
			viewModel.accountType = args.accountType
			when (args.accountType) {
				AccountType.Teacher -> {
					binding.fabCreateAssignment.visibility = VISIBLE
					binding.fabCreateAssignment.isEnabled = true
					binding.bDeleteClass.visibility = VISIBLE
					binding.bDeleteClass.isEnabled = true
				}
				else -> {}
			}
		} ?: run {
			Toast.makeText(context, R.string.assignments_load_fail, Toast.LENGTH_LONG).show()
			findNavController().popBackStack()
			return
		}

		initRecyclerView()
		initClickListeners()
		initObservers()

		sharedPrefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)
		val id = sharedPrefs.getInt("id", -1)
		if (id >= 0) {
			sharedPrefs.getString("password", null)?.also {
				viewModel.getClassAssignments(id, it)
			} ?: run {
				Toast.makeText(context, R.string.load_profile_fail, Toast.LENGTH_LONG).show()
				findNavController().popBackStack()
			}
		} else {
			Toast.makeText(context, R.string.internal_error, Toast.LENGTH_LONG).show()
			findNavController().popBackStack()
		}

	}

	private fun initRecyclerView() {

		adapter = AssignmentsAdapter(viewModel.assignmentDatas, this)
		binding.rvAssignments.adapter = adapter
		binding.rvAssignments.layoutManager = LinearLayoutManager(requireContext())

	}

	private fun initClickListeners() {

		binding.fabCreateAssignment.setOnClickListener {
			findNavController().navigate(R.id.action_assignmentsFragment_to_createAssignmentFragment, CreateAssignmentFragmentArgs(viewModel.classId).toBundle())
		}

		binding.bDeleteClass.setOnClickListener {
			AlertDialog.Builder(context)
				.setTitle(R.string.delete_question)
				.setMessage(R.string.delete_class_confirm)
				.setPositiveButton(R.string.yes) { _, _ ->
					deleteClass()
				}
				.setNegativeButton(R.string.no, null).show()
		}

	}

	private fun initObservers() {

		viewModel.getClassAssignments.observe(viewLifecycleOwner) {
			it?.also { newAssignments ->
				if (newAssignments.isEmpty()) {
					if (viewModel.accountType != AccountType.Teacher) {
						Toast.makeText(context, R.string.no_assignments, Toast.LENGTH_LONG).show()
					}
					val prevSize = adapter.itemCount
					viewModel.assignmentDatas.clear()
					adapter.notifyItemRangeRemoved(0, prevSize)
					return@also
				}
				val prevSize = viewModel.assignmentDatas.size
				viewModel.assignmentDatas.clear()
				adapter.notifyItemRangeRemoved(0, prevSize)
				viewModel.assignmentDatas.addAll(newAssignments)
				adapter.notifyItemRangeInserted(0, viewModel.assignmentDatas.size)
			}
		}

		viewModel.deleteClass.observe(viewLifecycleOwner) { status ->
			when (status) {
				200 -> {
					findNavController().popBackStack()
					return@observe
				}
				else -> {
					Log.d("DEBUG", "$status")
					Toast.makeText(context, R.string.delete_class_fail, Toast.LENGTH_LONG).show()
					binding.bDeleteClass.isEnabled = true
					binding.fabCreateAssignment.isEnabled = true
				}
			}
		}

	}

	private fun deleteClass() {

		val password = sharedPrefs.getString("password", null)
		password?.also {
			viewModel.deleteClass(it)
		} ?: run {
			Toast.makeText(context, R.string.internal_error, Toast.LENGTH_SHORT).show()
			findNavController().popBackStack()
			return
		}

		binding.bDeleteClass.isEnabled = false
		binding.fabCreateAssignment.isEnabled = false

	}

	fun onAssignmentClick(position: Int) {

		val assignment = viewModel.assignmentDatas[position]
		if (assignment.openTime != null && assignment.openTime > System.currentTimeMillis() / 1000L) {
			Toast.makeText(context, R.string.assignment_not_opened, Toast.LENGTH_LONG).show()
			return
		}
		if (assignment.closeTime != null && assignment.closeTime < System.currentTimeMillis() / 1000L) {
			Toast.makeText(context, R.string.assignment_already_closed, Toast.LENGTH_LONG).show()
			return
		}
		when (viewModel.accountType) {
			AccountType.Teacher -> {

			}
			AccountType.Student -> {
				findNavController().navigate(
					R.id.action_assignmentsFragment_to_qwiz_navigation,
					QwizFullPreviewFragmentArgs(assignment.qwizId, assignment.id).toBundle()
				)
			}
			else -> {}
		}

	}

}