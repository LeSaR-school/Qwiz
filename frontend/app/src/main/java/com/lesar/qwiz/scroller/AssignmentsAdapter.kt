package com.lesar.qwiz.scroller

import android.icu.text.SimpleDateFormat
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.lesar.qwiz.R
import com.lesar.qwiz.api.model.assignment.AssignmentData
import com.lesar.qwiz.fragment.ClassFragment
import java.util.Date

class AssignmentsAdapter(
	private var assignmentDatas: List<AssignmentData> = listOf(),
	var fragment: ClassFragment,
) : RecyclerView.Adapter<AssignmentsAdapter.ClassPreviewHolder>() {

	inner class ClassPreviewHolder(classView: View) : RecyclerView.ViewHolder(classView)

	override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ClassPreviewHolder {
		val view = LayoutInflater.from(parent.context).inflate(R.layout.view_assignment_preview, parent, false)
		val holder = ClassPreviewHolder(view)
		view.setOnClickListener {
			fragment.onAssignmentClick(holder.absoluteAdapterPosition)
		}
		return holder
	}

	override fun onBindViewHolder(holder: ClassPreviewHolder, position: Int) {
		holder.itemView.apply {

			val assignmentData = assignmentDatas[position]

			findViewById<TextView>(R.id.tvAssignmentQwizName).text = assignmentData.qwizName
			val pattern = "HH:mm\ndd/MM/yy"
			findViewById<TextView>(R.id.tvOpenTime).text = if (assignmentData.openTime != null) {
				val dt = Date(assignmentData.openTime * 1000)
				SimpleDateFormat(pattern, resources.configuration.locales.get(0)).format(dt)
			} else { "-" }
			findViewById<TextView>(R.id.tvCloseTime).text = if (assignmentData.closeTime != null) {
				val dt = Date(assignmentData.closeTime * 1000)
				SimpleDateFormat(pattern, resources.configuration.locales.get(0)).format(dt)
			} else { "-" }

		}
	}

	override fun getItemCount(): Int {
		return assignmentDatas.size
	}

}