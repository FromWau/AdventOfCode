package day1

import java.io.File

fun main() {

    try {
        val file = File("input.txt")
        val lines = file.readLines()

        val sum =
                lines
                        .map { line ->
                            var first: String = ""
                            for (i in 0..line.length) {
                                val sub = line.substring(0, i)

                                val digit = sub.firstOrNull { it.isDigit() }
                                if (digit != null) {
                                    first = digit.toString()
                                    break
                                }

                                val spelledDigit =
                                        sub.replaceSpelledNumbersWithDigits().firstOrNull {
                                            it.isDigit()
                                        }

                                if (spelledDigit != null) {
                                    first = spelledDigit.toString()
                                    break
                                }
                            }

                            if (first.isEmpty()) {
                                return@map 0
                            }

                            var second: String = ""
                            for (i in line.length downTo 0) {
                                val sub = line.substring(i, line.length)
                                val digit = sub.firstOrNull { it.isDigit() }

                                if (digit != null) {
                                    second = digit.toString()
                                    break
                                }

                                val spelledDigit =
                                        sub.replaceSpelledNumbersWithDigits().firstOrNull {
                                            it.isDigit()
                                        }

                                if (spelledDigit != null) {
                                    second = spelledDigit.toString()
                                    break
                                }
                            }

                            val number = "$first$second".toInt()

                            return@map number
                        }
                        .sum()
        println(sum)
    } catch (e: Exception) {
        println("Error occurred: ${e.message}")
    }
}

fun String.replaceSpelledNumbersWithDigits(): String =
        this.replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
