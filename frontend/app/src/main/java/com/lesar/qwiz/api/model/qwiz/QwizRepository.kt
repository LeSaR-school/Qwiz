package com.lesar.qwiz.api.model.qwiz

import com.lesar.qwiz.MediaEncoder
import com.lesar.qwiz.api.RetrofitProvider
import com.lesar.qwiz.api.model.account.AccountPasswordData
import com.lesar.qwiz.api.model.media.CreateMediaData
import com.lesar.qwiz.api.model.media.MediaType
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import retrofit2.Response

class QwizRepository {

	suspend fun getQwiz(id: Int): Qwiz? {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.qwizApi.getQwiz(id)
		}
	}

	suspend fun getAccountQwizes(id: Int, password: String): List<QwizPreview> {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.qwizApi.getAccountQwizes(id, AccountPasswordData(password))
		}
	}

	suspend fun getBestQwizPreviews(page: Int = 0, search: String? = null): List<QwizPreview> {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.qwizApi.getBestQwizPreviews(page, search)
		}
	}

	suspend fun getRecentQwizPreviews(page: Int = 0, search: String? = null): List<QwizPreview> {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.qwizApi.getRecentQwizPreviews(page, search)
		}
	}

	suspend fun solveQwiz(id: Int, username: String?, answers: List<Short>, assignmentId: Int): SolveQwizResponse? {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.qwizApi.solveQwiz(
				id,
				SolveQwizData(username, answers),
				if (assignmentId >= 0) { assignmentId } else { null }
			)
		}
	}

	suspend fun createQwiz(creatorId: Int, creatorPassword: String, name: String, thumbnailBytes: ByteArray?, questions: MutableList<CreateQuestionEditData>): Response<Void> {
		return withContext(Dispatchers.IO) {

			val createQuestionDatas = editDatasToDatas(questions)
			val qwizData = CreateQwizData(
				creatorPassword,
				QwizOnlyData(
					name,
					creatorId,
					thumbnailBytes?.let {
						CreateMediaData(MediaEncoder.encode(it), MediaType.Image)
					},
					true
				),
				createQuestionDatas
			)

			return@withContext RetrofitProvider.qwizApi.createQwiz(qwizData)

		}

	}

	suspend fun editQwiz(id: Int, creatorPassword: String, newName: String, newThumbnailBytes: ByteArray?): Response<Void> {
		return withContext(Dispatchers.IO) {

			val qwizData = EditQwizData(
				creatorPassword,
				newName,
				newThumbnailBytes?.let {
					CreateMediaData(
						MediaEncoder.encode(it),
						MediaType.Image,
					)
				}
			)

			return@withContext RetrofitProvider.qwizApi.editQwiz(id, qwizData)

		}

	}

	suspend fun updateQuestion(qwizId: Int, creatorPassword: String, index: Int, data: CreateQuestionEditData): Response<Void> {
		return withContext(Dispatchers.IO) {

			val answers = MutableList(4) { index -> NewAnswer((index + 1).toShort(), null) }
			for ((i, answer) in data.answers.withIndex()) {
				answers[i].content = answer
			}

			return@withContext RetrofitProvider.qwizApi.updateQuestion(
				qwizId,
				index,
				UpdateQuestionData(
					creatorPassword,
					data.body,
					answers,
					data.correct,
					data.embedBytes?.let { bytes ->
						CreateMediaData(
							MediaEncoder.encode(bytes),
							MediaType.Image,
						)
					}
				)
			)

		}

	}

	suspend fun deleteQwiz(id: Int, password: String): Response<Void> {
		return withContext(Dispatchers.IO) {
			return@withContext RetrofitProvider.qwizApi.deleteQwiz(
				id,
				DeleteQwizData(password)
			)
		}
	}



	private fun editDatasToDatas(questions: MutableList<CreateQuestionEditData>): List<CreateQuestionData> {
		return questions.map { editDataToData(it) }
	}

	private fun editDataToData(question: CreateQuestionEditData): CreateQuestionData {
		return CreateQuestionData(
			question.body,
			question.answers[0],
			question.answers[1],
			question.answers.getOrNull(2),
			question.answers.getOrNull(3),
			question.correct,
			question.embedBytes?.let { data ->
				CreateMediaData(
					MediaEncoder.encode(data),
					MediaType.Image,
				)
			},
		)
	}

}