package day3

import java.io.File

fun main() {
    part2()
}

fun part1() {
    val schematic: List<String> = File("input.txt").readLines()

    val partNumbers = mutableListOf<Int>()

    schematic.forEachIndexed { lineIndex, line ->
        var number: Int = 0
        var hasSymbolasNeighbor = false
        line.forEachIndexed { charIndex, char ->
            if (char.isDigit()) {
                number = number * 10 + char.digitToInt()

                val x =
                        hasPositionASymbolAsNeighbor(
                                pos = Pair(lineIndex, charIndex),
                                schematic = schematic
                        )
                hasSymbolasNeighbor = x || hasSymbolasNeighbor

                if (charIndex == line.length - 1) {
                    if (hasSymbolasNeighbor) {
                        partNumbers.add(number)
                    }

                    number = 0
                    hasSymbolasNeighbor = false
                }
            } else {
                if (hasSymbolasNeighbor) {
                    partNumbers.add(number)
                }

                number = 0
                hasSymbolasNeighbor = false
            }
        }
    }

    println("Part 1: ${partNumbers.sum()}")
}

fun part2() {
    val schematic: List<String> = File("input.txt").readLines()

    val partNumbers = mutableMapOf<Pair<Int, Int>, List<Int>>()

    schematic.forEachIndexed { lineIndex, line ->
        var number: Int = 0
        var position: Pair<Int, Int>? = null
        line.forEachIndexed { charIndex, char ->
            if (char.isDigit()) {
                number = number * 10 + char.digitToInt()

                position =
                        pt2hasPositionASymbolAsNeighbor(
                                pos = Pair(lineIndex, charIndex),
                                schematic = schematic
                        ) ?: position

                if (charIndex == line.length - 1) {
                    position?.let {
                        partNumbers[it] = partNumbers[it]?.plus(number) ?: listOf(number)
                    }
                    number = 0
                    position = null
                }
            } else {
                position?.let { partNumbers[it] = partNumbers[it]?.plus(number) ?: listOf(number) }
                number = 0
                position = null
            }
        }
    }
    
    val sum = partNumbers
            .filterKeys { partNumbers[it]?.size == 2 }
            .values
            .map { it.reduce { acc, i -> acc * i } }
            .sum()

    println("Part 2: $sum")
}

fun hasPositionASymbolAsNeighbor(pos: Pair<Int, Int>, schematic: List<String>): Boolean {
    for (i in -1..1) {
        for (j in -1..1) {
            if (i == 0 && j == 0) {
                continue
            }

            val neighbor: Char =
                    try {
                        schematic[pos.first + i][pos.second + j]
                    } catch (_: Exception) {
                        continue
                    }

            if (neighbor.isSymbol()) {
                return true
            }
        }
    }

    return false
}

fun Char.isSymbol(): Boolean = !this.isLetter() && !this.isDigit() && this != '.'

fun pt2hasPositionASymbolAsNeighbor(pos: Pair<Int, Int>, schematic: List<String>): Pair<Int, Int>? {
    for (i in -1..1) {
        for (j in -1..1) {
            if (i == 0 && j == 0) {
                continue
            }

            val neighbor: Char =
                    try {
                        schematic[pos.first + i][pos.second + j]
                    } catch (_: Exception) {
                        continue
                    }

            if (neighbor.pt2isSymbol()) {
                return Pair(pos.first + i, pos.second + j)
            }
        }
    }

    return null
}

fun Char.pt2isSymbol(): Boolean = this == '*'
