package com.lesar.qwiz.fragment

import android.app.AlertDialog
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
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
import com.lesar.qwiz.api.BASE_URL
import com.lesar.qwiz.api.model.account.AccountType
import com.lesar.qwiz.databinding.FragmentProfileBinding
import com.lesar.qwiz.scroller.ClassPreviewsAdapter
import com.lesar.qwiz.viewmodel.ProfileViewModel
import com.squareup.picasso.Picasso


class ProfileFragment : Fragment(R.layout.fragment_profile) {

	private lateinit var binding: FragmentProfileBinding
	
	private val viewModel: ProfileViewModel by viewModels()

	private lateinit var sharedPrefs: SharedPreferences

	private lateinit var adapter: ClassPreviewsAdapter


	
	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentProfileBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		sharedPrefs = requireActivity().getSharedPreferences("user", Context.MODE_MULTI_PROCESS)
		val id = sharedPrefs.getInt("id", -1)

		if (id >= 0) {
			viewModel.getAccount(id)
		} else {
			findNavController().navigate(R.id.action_profileFragment_to_loginFragment)
		}

		initRecyclerView()
		initClickListeners()
		initObservers()
	}

	private fun initRecyclerView() {

		adapter = ClassPreviewsAdapter(viewModel.classDatas, this)
		binding.rvClassPreviews.adapter = adapter
		binding.rvClassPreviews.layoutManager = LinearLayoutManager(requireContext())

	}

	private fun initClickListeners() {

		binding.bSignOut.setOnClickListener {
			val editor = sharedPrefs.edit()
			editor.remove("id")
			editor.remove("username")
			editor.remove("password")
			editor.apply()
			requireActivity().finish()
		}

		binding.bEditProfile.setOnClickListener {
			sharedPrefs.getString("password", null)?.also {
				val args = ProfileEditFragmentArgs(viewModel.account.id, viewModel.account.username, viewModel.account.accountType, it)
				findNavController().navigate(
					R.id.action_profileFragment_to_profileEditFragment,
					args.toBundle()
				)
			}
		}

		binding.bDeleteAccount.setOnClickListener {
			AlertDialog.Builder(context)
				.setTitle(R.string.delete_question)
				.setMessage(R.string.delete_account_confirm)
				.setPositiveButton(R.string.yes) { _, _ ->
					deleteAccount()
				}
				.setNegativeButton(R.string.no, null).show()
		}

		binding.fabCreateClass.setOnClickListener {
			findNavController().navigate(R.id.action_profileFragment_to_createClassFragment)
		}

	}


	private fun initObservers() {

		viewModel.getAccount.observe(viewLifecycleOwner) {
			it?.also {
				viewModel.account = it

				binding.tvProfileUsername.text = it.username
				binding.tvProfileAccountType.text = it.accountType.toString()
				it.profilePicture?.let { media ->
					Picasso.get()
						.load("$BASE_URL${media.uri}")
						.into(binding.ivProfilePicture)
				}

				binding.bEditProfile.isEnabled = true

				sharedPrefs.getString("password", null)?.let { password ->
					if (it.accountType == AccountType.Student || it.accountType == AccountType.Teacher) viewModel.getAccountClasses(it.id, password)
				}
				if (it.accountType == AccountType.Teacher) {
					binding.fabCreateClass.visibility = VISIBLE
					binding.fabCreateClass.isEnabled = true
				}
			} ?: run {
				Toast.makeText(context, R.string.load_profile_fail, Toast.LENGTH_LONG).show()
				requireActivity().finish()
			}
		}

		viewModel.deleteAccount.observe(viewLifecycleOwner) {
			when (it) {
				200 -> {
					Toast.makeText(context, R.string.delete_account_success, Toast.LENGTH_LONG).show()
					deletePrefs()
					requireActivity().finish()
				}

				401 -> {
					Toast.makeText(context, R.string.password_incorrect, Toast.LENGTH_LONG).show()
				}

				404, 500 -> {
					Toast.makeText(context, R.string.internal_error, Toast.LENGTH_LONG).show()
				}
			}

			binding.bDeleteAccount.isEnabled = true
			binding.bSignOut.isEnabled = true
			binding.bEditProfile.isEnabled = true
		}

		viewModel.getAccountClasses.observe(viewLifecycleOwner) {
			it?.also {
				val prevSize = viewModel.classDatas.size
				viewModel.classDatas.clear()
				adapter.notifyItemRangeRemoved(0, prevSize)
				viewModel.classDatas.addAll(it)
				adapter.notifyItemRangeInserted(0, it.size)
			} ?: run {
				Toast.makeText(context, R.string.load_classes_fail, Toast.LENGTH_LONG).show()
			}
		}

	}

	private fun deleteAccount() {
		val id = sharedPrefs.getInt("id", -1)
		val password = sharedPrefs.getString("password", null)
		password?.let {
			viewModel.deleteAccount(id, it)

			binding.bDeleteAccount.isEnabled = false
			binding.bSignOut.isEnabled = false
			binding.bEditProfile.isEnabled = false
		}
	}

	private fun deletePrefs() {
		val editor = sharedPrefs.edit()
		editor.remove("id")
		editor.remove("username")
		editor.remove("password")
		editor.apply()
	}

	fun onClassClick(position: Int) {
		val classData = viewModel.classDatas[position]
		findNavController().navigate(R.id.action_profileFragment_to_classFragment, ClassFragmentArgs(classData.id, viewModel.account.accountType, classData.name).toBundle())
	}

}