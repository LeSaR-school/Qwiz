package com.lesar.qwiz.fragment

import android.app.AlertDialog
import android.content.Context.MODE_PRIVATE
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import androidx.recyclerview.widget.LinearLayoutManager
import com.lesar.qwiz.R
import com.lesar.qwiz.databinding.FragmentQwizPreviewsBinding
import com.lesar.qwiz.scroller.QwizPreviewsAdapter
import com.lesar.qwiz.viewmodel.QwizPreviewsViewModel

class QwizPreviewsFragment : Fragment(R.layout.fragment_qwiz_previews) {

	private lateinit var binding: FragmentQwizPreviewsBinding

	private val viewModel: QwizPreviewsViewModel by viewModels()

	private lateinit var adapter: QwizPreviewsAdapter



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentQwizPreviewsBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

		super.onViewCreated(view, savedInstanceState)

		val sharedPreferences = requireActivity().getSharedPreferences("user", MODE_PRIVATE)
		if (sharedPreferences.getString("password", null) != null) {
			binding.fabCreateQwiz.isEnabled = true
		}

		initRecyclerView()
		initClickListeners()
		initObservers()

		searchQwizes()

	}

	private fun initRecyclerView() {

		adapter = QwizPreviewsAdapter(viewModel.qwizPreviews, this)
		binding.rvQwizPreviews.adapter = adapter
		binding.rvQwizPreviews.layoutManager = LinearLayoutManager(requireContext())

	}

	private fun initClickListeners() {

		binding.fabCreateQwiz.setOnClickListener {
			findNavController().navigate(R.id.action_qwizPreviewsFragment_to_qwiz_create_navigation)
		}

		binding.bFilterQwizes.setOnClickListener {
			var which: Int = when (viewModel.sortBy) {
				QwizPreviewsViewModel.SortBy.Votes -> 0
				QwizPreviewsViewModel.SortBy.Recent -> 1
			}
			val builder: AlertDialog.Builder = AlertDialog.Builder(context)
			builder
				.setTitle(R.string.sort_by)
				.setPositiveButton(R.string.ok) { _, _ ->
					viewModel.sortBy = when (which) {
						0 -> QwizPreviewsViewModel.SortBy.Votes
						1 -> QwizPreviewsViewModel.SortBy.Recent
						else -> QwizPreviewsViewModel.SortBy.Votes
					}
					searchQwizes()
				}
				.setNegativeButton(R.string.cancel) {_,_->}
				.setSingleChoiceItems(arrayOf(getString(R.string.by_votes), getString(R.string.by_recent)), which) { _, newWhich -> which = newWhich }

			builder.create().show()
		}

		binding.bSearchQwizes.setOnClickListener {
			searchQwizes()
		}

	}

	private fun initObservers() {

		viewModel.getQwizPreviews.observe(viewLifecycleOwner) {
			it?.let { newPreviews ->
				val prevSize = viewModel.qwizPreviews.size
				viewModel.qwizPreviews.clear()
				adapter.notifyItemRangeRemoved(0, prevSize)
				viewModel.qwizPreviews.addAll(newPreviews)
				adapter.notifyItemRangeInserted(0, newPreviews.size)
			}
		}

	}

	private fun searchQwizes() {
		val search = binding.etSearchQwizes.text.toString()
		when(viewModel.sortBy) {
			QwizPreviewsViewModel.SortBy.Votes -> {
				viewModel.getBestQwizPreviews(0, search)
			}
			QwizPreviewsViewModel.SortBy.Recent -> {
				viewModel.getRecentQwizPreviews(0, search)
			}
		}
	}

	fun onQwizPreviewClick(position: Int) {
		val id = viewModel.qwizPreviews[position].id
		findNavController().navigate(
			R.id.action_qwizPreviewsFragment_to_qwiz_navigation,
			QwizFullPreviewFragmentArgs(id).toBundle()
		)
	}

}