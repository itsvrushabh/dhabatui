package com.dhabapos.dhabatui.network

import com.dhabapos.dhabatui.model.DashboardData
import com.google.gson.Gson
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.OkHttpClient
import okhttp3.Request
import java.util.concurrent.TimeUnit

enum class ConnectionStatus {
    CONNECTED,
    CONNECTING,
    DISCONNECTED
}

class DhabaApiClient(
    private var baseUrl: String = "http://192.168.1.10:8080"
) {
    private val client = OkHttpClient.Builder()
        .connectTimeout(3, TimeUnit.SECONDS)
        .readTimeout(3, TimeUnit.SECONDS)
        .build()

    private val gson = Gson()
    private var lastCachedData: DashboardData? = null

    fun updateBaseUrl(ip: String, port: Int = 8080) {
        val cleanIp = ip.trim().removePrefix("http://").removePrefix("https://").trimEnd('/')
        baseUrl = "http://$cleanIp:$port"
    }

    fun getBaseUrl(): String = baseUrl

    suspend fun fetchStatus(): Result<DashboardData> = withContext(Dispatchers.IO) {
        try {
            val url = "$baseUrl/api/status"
            val request = Request.Builder()
                .url(url)
                .header("Accept", "application/json")
                .build()

            client.newCall(request).execute().use { response ->
                if (!response.isSuccessful) {
                    return@withContext Result.failure(Exception("HTTP Error: ${response.code}"))
                }
                val body = response.body?.string() ?: return@withContext Result.failure(Exception("Empty response body"))
                val data = gson.fromJson(body, DashboardData::class.java)
                lastCachedData = data
                Result.success(data)
            }
        } catch (e: Exception) {
            // Return cached data if available, but wrap in failure
            Result.failure(e)
        }
    }

    fun getLastCachedData(): DashboardData? = lastCachedData
}
