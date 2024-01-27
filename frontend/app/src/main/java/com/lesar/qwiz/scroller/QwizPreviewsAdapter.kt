package com.lesar.qwiz.scroller

import android.icu.text.SimpleDateFormat
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.lesar.qwiz.R
import com.lesar.qwiz.api.BASE_URL
import com.lesar.qwiz.api.model.qwiz.QwizPreview
import com.lesar.qwiz.fragment.QwizPreviewsFragment
import com.squareup.picasso.Picasso
import java.util.Date

class QwizPreviewsAdapter(
	private var qwizPreviews: List<QwizPreview> = listOf(),
	var fragment: QwizPreviewsFragment,
) : RecyclerView.Adapter<QwizPreviewsAdapter.QwizPreviewHolder>() {

	inner class QwizPreviewHolder(qwizView: View) : RecyclerView.ViewHolder(qwizView)

	override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): QwizPreviewHolder {
		val view = LayoutInflater.from(parent.context).inflate(R.layout.view_qwiz_preview, parent, false)
		val holder = QwizPreviewHolder(view)
		view.setOnClickListener {
			fragment.onQwizPreviewClick(holder.absoluteAdapterPosition)
		}
		return holder
	}

	override fun onBindViewHolder(holder: QwizPreviewHolder, position: Int) {
		holder.itemView.apply {
			val qwizData = qwizPreviews[position]

			findViewById<TextView>(R.id.tvQwizPreviewName).text = qwizData.name
			findViewById<TextView>(R.id.tvQwizPreviewCreator).text = qwizData.creatorName
			findViewById<TextView>(R.id.tvPreviewVotesNumber).text = qwizData.votes.toString()

			val dt = Date(qwizData.createTime)
			val formattedTime = SimpleDateFormat("dd/MM/yy", resources.configuration.locales.get(0)).format(dt)
			findViewById<TextView>(R.id.tvCreateTime).text = formattedTime

			qwizData.thumbnailUri?.let {
				Picasso.get()
					.load("$BASE_URL$it")
					.into(findViewById<ImageView>(R.id.ivQwizPreviewThumbnail))
			}
		}
	}

	override fun getItemCount(): Int {
		return qwizPreviews.size
	}

}