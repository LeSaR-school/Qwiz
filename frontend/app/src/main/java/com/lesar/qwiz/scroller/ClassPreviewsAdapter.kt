package com.lesar.qwiz.scroller

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.lesar.qwiz.R
import com.lesar.qwiz.api.model.group.ClassData
import com.lesar.qwiz.fragment.ProfileFragment

class ClassPreviewsAdapter(
	private var classDatas: List<ClassData> = listOf(),
	var fragment: ProfileFragment,
) : RecyclerView.Adapter<ClassPreviewsAdapter.ClassPreviewHolder>() {


	inner class ClassPreviewHolder(classView: View) : RecyclerView.ViewHolder(classView)

	override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ClassPreviewHolder {
		val view = LayoutInflater.from(parent.context).inflate(R.layout.view_class_preview, parent, false)
		val holder = ClassPreviewHolder(view)
		view.setOnClickListener {
			fragment.onClassClick(holder.absoluteAdapterPosition)
		}
		return holder
	}

	override fun onBindViewHolder(holder: ClassPreviewHolder, position: Int) {
		holder.itemView.apply {
			val classData = classDatas[position]

			findViewById<TextView>(R.id.tvClassPreviewName).text = classData.name
			findViewById<TextView>(R.id.tvClassPreviewTeacher).text = classData.teacherName
			findViewById<TextView>(R.id.tvStudentsNumber).text = classData.studentIds.size.toString()
		}
	}

	override fun getItemCount(): Int {
		return classDatas.size
	}

}