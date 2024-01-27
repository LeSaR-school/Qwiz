package com.lesar.qwiz.fragment

import android.content.Context.MODE_MULTI_PROCESS
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.navigation.fragment.findNavController
import com.lesar.qwiz.R
import com.lesar.qwiz.databinding.FragmentLoginBinding
import com.lesar.qwiz.viewmodel.LoginViewModel

class LoginFragment : Fragment(R.layout.fragment_login) {

	private lateinit var binding: FragmentLoginBinding

	private val viewModel: LoginViewModel by viewModels()



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentLoginBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		binding.bRegister.setOnClickListener {
			activity?.let {
				findNavController().navigate(R.id.action_loginFragment_to_registerFragment)
			}
		}

		binding.bLogin.setOnClickListener {
			if (binding.etUsername.text.isEmpty()) {
				binding.etUsername.requestFocus()
				Toast.makeText(requireContext(), R.string.enter_username, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			}
			if (binding.etPassword.text.isEmpty()) {
				binding.etPassword.requestFocus()
				Toast.makeText(requireContext(), R.string.enter_password, Toast.LENGTH_SHORT).show()
				return@setOnClickListener
			}

			viewModel.verifyPassword(binding.etUsername.text.toString(), binding.etPassword.text.toString())

			binding.bLogin.isEnabled = false
			binding.bRegister.isEnabled = false
		}

		initObservers()
	}

	private fun initObservers() {
		viewModel.verifyPassword.observe(viewLifecycleOwner) {
			it?.also {
				when (it.code) {
					200 -> {
						it.account?.let { account ->
							savePrefs(account.id, account.username, it.password)
						}
						activity?.finish()
						return@observe
					}

					404 -> {
						binding.etUsername.requestFocus()
						Toast.makeText(requireContext(), R.string.username_not_found, Toast.LENGTH_SHORT)
							.show()
					}

					401 -> {
						binding.etPassword.requestFocus()
						Toast.makeText(requireContext(), R.string.password_incorrect, Toast.LENGTH_SHORT)
							.show()
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
				Toast.makeText(requireContext(), R.string.login_fail, Toast.LENGTH_LONG).show()
			}

			binding.bLogin.isEnabled = true
			binding.bRegister.isEnabled = true
		}
	}

	private fun savePrefs(id: Int, username: String, password: String) {
		val sharedRefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)
		val editor = sharedRefs.edit()
		editor.putInt("id", id)
		editor.putString("username", username)
		editor.putString("password", password)
		editor.apply()
	}
}