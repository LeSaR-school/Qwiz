package com.lesar.qwiz.fragment

import android.content.Context
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.RadioButton
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import com.lesar.qwiz.R
import com.lesar.qwiz.api.model.account.AccountType
import com.lesar.qwiz.databinding.FragmentRegisterBinding
import com.lesar.qwiz.viewmodel.RegisterViewModel

class RegisterFragment : Fragment(R.layout.fragment_register) {

	private lateinit var binding: FragmentRegisterBinding

	private val viewModel: RegisterViewModel by viewModels()



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentRegisterBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		initClickListeners()
		initObservers()
	}

	private fun initClickListeners() {

		binding.bLogin.setOnClickListener {
			activity?.let {
				findNavController().popBackStack()
			}
		}

		binding.bRegister.setOnClickListener {
			val checkedAccountType = binding.rgNewAccountType.checkedRadioButtonId
			val rbCheckedAccount = requireView().findViewById<RadioButton>(checkedAccountType)

			if (binding.etUsername.text.isEmpty()) {
				binding.etUsername.requestFocus()
				Toast.makeText(requireContext(), R.string.enter_username, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			} else if (binding.etPassword.text.isEmpty()) {
				binding.etPassword.requestFocus()
				Toast.makeText(requireContext(), R.string.enter_password, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			} else if (rbCheckedAccount == null) {
				binding.rgNewAccountType.requestFocus()
				Toast.makeText(requireContext(), R.string.select_account_type, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			}

			if (!validateUsername(binding.etUsername.text.toString())) {
				binding.etUsername.requestFocus()
				return@setOnClickListener
			}
			if (!validatePassword(binding.etPassword.text.toString())) {
				binding.etPassword.requestFocus()
				return@setOnClickListener
			}
			if (binding.etPassword.text.toString() != binding.etPasswordConfirm.text.toString()) {
				binding.etPasswordConfirm.requestFocus()
				Toast.makeText(requireContext(), R.string.passwords_dont_match, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			}

			viewModel.createAccount(binding.etUsername.text.toString(), binding.etPassword.text.toString(), AccountType.valueOf(rbCheckedAccount.text.toString()))

			binding.bLogin.isEnabled = false
			binding.bRegister.isEnabled = false
		}

	}

	private fun initObservers() {

		viewModel.createAccount.observe(viewLifecycleOwner) {
			it?.also {
				when (it.code) {
					201 -> {
						it.account?.let { account ->
							savePrefs(account.id, account.username, it.password)
						}
						activity?.finish()
						return@observe
					}

					409 -> {
						binding.etUsername.requestFocus()
						Toast.makeText(requireContext(), R.string.username_taken, Toast.LENGTH_SHORT).show()
					}

					500 -> {
						Toast.makeText(
							requireContext(),
							R.string.internal_error,
							Toast.LENGTH_SHORT
						).show()
					}
				}
			} ?: run {
				Toast.makeText(requireContext(), R.string.register_fail, Toast.LENGTH_LONG).show()
			}

			binding.bRegister.isEnabled = true
			binding.bLogin.isEnabled = true
		}

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

	private fun savePrefs(id: Int, username: String, password: String) {
		val sharedRefs = requireActivity().getSharedPreferences("user", Context.MODE_MULTI_PROCESS)
		val editor = sharedRefs.edit()
		editor.putInt("id", id)
		editor.putString("username", username)
		editor.putString("password", password)
		editor.apply()
	}

}