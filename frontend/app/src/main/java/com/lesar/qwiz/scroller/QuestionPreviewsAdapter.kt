package com.lesar.qwiz.scroller

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.google.android.material.floatingactionbutton.FloatingActionButton
import com.lesar.qwiz.R
import com.lesar.qwiz.api.model.qwiz.CreateQuestionEditData
import com.lesar.qwiz.fragment.CreateQwizFragment

class QuestionPreviewsAdapter(
	private var questionPreviews: List<CreateQuestionEditData> = listOf(),
	var fragment: CreateQwizFragment
) : RecyclerView.Adapter<QuestionPreviewsAdapter.QuestionPreviewHolder>() {

	inner class QuestionPreviewHolder(view: View) : RecyclerView.ViewHolder(view)

	override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): QuestionPreviewHolder {
		val view = LayoutInflater.from(parent.context).inflate(
			if (viewType == 0) { R.layout.view_question_preview }
			else { R.layout.view_add_question },
			parent,
			false
		)

		val holder = QuestionPreviewHolder(view)
		if (viewType == 0) {
			view.setOnClickListener {
				fragment.editQuestion(holder.absoluteAdapterPosition)
			}
		} else {
			view.findViewById<FloatingActionButton>(R.id.fabAddQuestion).setOnClickListener {
				fragment.createQuestion()
			}
		}
		return holder
	}

	override fun getItemViewType(position: Int): Int {
		return if (position < itemCount - 1) { 0 } else { 1 }
	}

	override fun onBindViewHolder(holder: QuestionPreviewHolder, position: Int) {
		if (position == itemCount - 1) return

		holder.itemView.apply {
			val questionData = questionPreviews[position]

			findViewById<TextView>(R.id.tvQuestionBodyPreview).text = questionData.body
			findViewById<TextView>(R.id.tvStudentsNumber).text = questionData.answers.size.toString()
		}
	}

	override fun getItemCount(): Int {
		return questionPreviews.size + 1
	}

}