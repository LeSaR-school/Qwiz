package com.lesar.qwiz.fragment

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.provider.MediaStore
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.RadioButton
import android.widget.Toast
import androidx.activity.result.ActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import com.lesar.qwiz.MediaEncoder
import com.lesar.qwiz.R
import com.lesar.qwiz.api.model.account.AccountType
import com.lesar.qwiz.databinding.FragmentProfileEditBinding
import com.lesar.qwiz.viewmodel.ProfileEditViewModel


class ProfileEditFragment : Fragment(R.layout.fragment_profile_edit) {

	private lateinit var binding: FragmentProfileEditBinding

	private val viewModel: ProfileEditViewModel by viewModels()



	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		viewModel.resultLauncher = registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { handleNewProfilePicture(it) }
	}

	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentProfileEditBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		arguments?.also {
			viewModel.args = ProfileEditFragmentArgs.fromBundle(it)

			binding.etNewUsername.setText(viewModel.args.username)
			(when (viewModel.args.accountType) {
				AccountType.Student -> binding.rbStudent
				AccountType.Parent -> binding.rbParent
				AccountType.Teacher -> binding.rbTeacher
			}).isChecked = true
		} ?: run {
			findNavController().popBackStack()
		}

		initClickListeners()
		initObservers()
	}

	private fun initClickListeners() {
		binding.bApplyChanges.setOnClickListener {
			applyChanges()
		}

		binding.ivProfilePicture.setOnClickListener {
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

	private fun initObservers() {
		viewModel.updateAccount.observe(viewLifecycleOwner) {
			it?.also {
				Log.d("DEBUG", "${it.code}")
				when (it.code) {
					200 -> {
						savePrefs(it.newUsername, it.newPassword)
						activity?.finish()
						return@observe
					}

					409 -> {
						Toast.makeText(
							requireContext(),
							R.string.username_taken,
							Toast.LENGTH_SHORT
						).show()
					}

					401, 500 -> {
						Toast.makeText(
							requireContext(),
							R.string.internal_error,
							Toast.LENGTH_SHORT
						).show()
					}
				}
			} ?: run {
				Toast.makeText(context, R.string.update_profile_fail, Toast.LENGTH_LONG).show()
				requireActivity().finish()
			}

			binding.bApplyChanges.isEnabled = true
		}
	}

	private fun applyChanges() {
		val newUsername = binding.etNewUsername.text.toString().let {
			if (it != viewModel.args.username) it
			else null
		}
		val newPassword = binding.etNewPassword.text.toString().let {
			if (it != viewModel.args.password && it.isNotEmpty()) it
			else null
		}
		val newAccountType = AccountType.valueOf(requireView().findViewById<RadioButton>(binding.rgNewAccountType.checkedRadioButtonId).text.toString()).let {
			if (it != viewModel.args.accountType) it
			else null
		}
		val newProfilePictureBase64 = viewModel.newProfilePictureBytes?.let {
			MediaEncoder.encode(it)
		}

		newUsername?.let {
			if (!validateUsername(it)) {
				binding.etNewUsername.requestFocus()
				return
			}
		}
		newPassword?.let {
			if (!validatePassword(it)) {
				binding.etNewPassword.requestFocus()
				return
			}

			if (it != binding.etNewPasswordConfirm.text.toString()) {
				binding.etNewPasswordConfirm.requestFocus()
				Toast.makeText(requireContext(), R.string.passwords_dont_match, Toast.LENGTH_SHORT).show()
				return
			}
		}

		viewModel.updateAccount(viewModel.args.id, viewModel.args.password, newUsername, newPassword, newAccountType, newProfilePictureBase64)

		binding.bApplyChanges.isEnabled = false
	}

	private fun validateUsername(username: String): Boolean {
		if (username.length < 3 || username.length > 12) {
			Toast.makeText(requireContext(), R.string.username_invalid_length, Toast.LENGTH_LONG).show()
			return false
		}
		if (!username.all { it.isLetterOrDigit() || it == '_' }) {
			Toast.makeText(requireContext(), R.string.username_invalid_chars, Toast.LENGTH_LONG).show()
			return false
		}

		return true
	}

	private fun validatePassword(password: String): Boolean {
		if (password.length < 8) {
			Toast.makeText(requireContext(), R.string.password_invalid_length, Toast.LENGTH_LONG).show()
			return false
		}
		if (password.filter { it.isLetter() }.length == password.filter { it.isLetter() && it.isUpperCase()}.length ) {
			Toast.makeText(requireContext(), R.string.password_lowercase, Toast.LENGTH_LONG).show()
			return false
		}
		if (password.filter { it.isLetter() }.length == password.filter { it.isLetter() && it.isLowerCase()}.length ) {
			Toast.makeText(requireContext(), R.string.password_uppercase, Toast.LENGTH_LONG).show()
			return false
		}
		if (password.none { it.isDigit() }) {
			Toast.makeText(requireContext(), R.string.password_digit, Toast.LENGTH_LONG).show()
			return false
		}
		if (password.none { !it.isLetterOrDigit() }) {
			Toast.makeText(requireContext(), R.string.password_special_char, Toast.LENGTH_LONG).show()
			return false
		}

		return true
	}

	private fun savePrefs(username: String?, password: String?) {
		val sharedPrefs = requireActivity().getSharedPreferences("user", Context.MODE_PRIVATE)
		val editor = sharedPrefs.edit()
		username?.let {
			editor.putString("username", it)
		}
		password?.let {
			editor.putString("password", it)
		}
		editor.apply()
	}

	private fun handleNewProfilePicture(res: ActivityResult) {
		if (res.resultCode != Activity.RESULT_OK) {
			Log.d("DEBUG", "${res.resultCode}")
			return
		}
		res.data?.data?.also { uri ->
			val fileStream = requireActivity().contentResolver.openInputStream(uri)
			fileStream?.also { stream ->
				viewModel.newProfilePictureBytes = stream.readBytes()
				fileStream.close()

				binding.ivProfilePicture.setImageURI(uri)
			} ?: run {
				Log.d("DEBUG", "file not found")
			}
		} ?: run {
			Log.d("DEBUG", "no uri selected")
		}
	}

}