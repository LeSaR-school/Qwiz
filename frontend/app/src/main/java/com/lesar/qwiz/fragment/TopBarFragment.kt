package com.lesar.qwiz.fragment

import android.content.Context.MODE_MULTI_PROCESS
import android.content.Intent
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import com.lesar.qwiz.AccountActivity
import com.lesar.qwiz.R
import com.lesar.qwiz.api.BASE_URL
import com.lesar.qwiz.databinding.FragmentTopBarBinding
import com.lesar.qwiz.viewmodel.TabBarViewModel
import com.squareup.picasso.Picasso

class TopBarFragment : Fragment(R.layout.fragment_top_bar) {

	private lateinit var binding: FragmentTopBarBinding

	private val viewModel: TabBarViewModel by viewModels()



	override fun onCreateView(
		inflater: LayoutInflater,
		container: ViewGroup?,
		savedInstanceState: Bundle?
	): View {
		binding = FragmentTopBarBinding.inflate(inflater, container, false)
		return binding.root
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)

		loadSharedPrefs()
		initClickListeners()
		initObservers()
	}

	override fun onResume() {
		super.onResume()

		loadSharedPrefs()
	}

	private fun loadSharedPrefs() {

		val sharedPrefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)

		val id = sharedPrefs.getInt("id", -1)
		val password = sharedPrefs.getString("password", null)

		if (id >= 0 && password != null) viewModel.verifyPassword(id, password)
		else {
			binding.tvUsername.setText(R.string.login)
			binding.ivProfile.setImageResource(R.drawable.profile)
		}

	}

	private fun initClickListeners() {

		binding.clProfile.setOnClickListener {
			val intent = Intent(requireActivity(), AccountActivity::class.java)
			requireActivity().startActivity(intent)
		}

	}

	private fun initObservers() {

		viewModel.verifyPassword.observe(viewLifecycleOwner) {
			it?.let {
				if (it.code == 200) {
					it.account?.let { account ->
						binding.tvUsername.text = account.username

						account.profilePicture?.let { media ->
							Picasso.get()
								.load("$BASE_URL${media.uri}")
								.into(binding.ivProfile)
						}

						val sharedPrefs = requireActivity().getSharedPreferences("user", MODE_MULTI_PROCESS)
						sharedPrefs.edit().putString("username", account.username).apply()
					}
				}
			} ?: run {
				Toast.makeText(context, R.string.load_profile_fail, Toast.LENGTH_LONG).show()
			}
		}

	}

}