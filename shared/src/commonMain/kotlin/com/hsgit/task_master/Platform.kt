package com.hsgit.task_master

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform