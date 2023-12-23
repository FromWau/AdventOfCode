package day4

import java.io.File
import kotlin.collections.mutableMapOf
import kotlin.math.pow

fun main() {
    val cards: MutableList<Card> =
            File("input.txt")
                    .readLines()
                    .filter { it.isNotBlank() }
                    .map {
                        val id = it.split(":")[0].split(" ").last().toInt()

                        val numbers = it.split(":")[1]

                        val winNo =
                                numbers.split("|")[0]
                                        .split(" ")
                                        .filter { it.isNotBlank() }
                                        .map { it.toInt() }
                                        .toList()
                        val playerNo =
                                numbers.split("|")[1]
                                        .split(" ")
                                        .filter { it.isNotBlank() }
                                        .map { it.toInt() }
                                        .toList()

                        Card(id, winNo, playerNo)
                    }
                    .toMutableList()

    cards.sumOf { it.getPoints() }.let { println("Part1: $it") }

    val cardMap = mutableMapOf<Card, Int>()
    cardMap.putAll(cards.map { it to 1 })

    cardMap.forEach { card ->
        repeat(card.value) {
            val newCards = card.key.getNewCards(cards)

            newCards.forEach { cardMap[it] = cardMap[it]!! + 1 }
        }
    }

    cardMap
            // .onEach { println("${it.value} instances of Card-${it.key.id}") }
            .map { it.value }
            .sum()
            .let { println("Part2: $it") }
}

data class Card(val id: Int, val winningNumbers: List<Int>, val playerNumbers: List<Int>) {
    fun getPoints(): Int {
        val count = playerNumbers.count { it in winningNumbers }
        if (count == 1) return 1

        return 2.0.pow(count - 1).toInt()
    }

    private fun countMatches(): Int {
        return playerNumbers.count { it in winningNumbers }
    }

    fun getNewCards(cards: List<Card>): List<Card> {
        val newCards: MutableList<Card> = mutableListOf()

        for (i in 0..countMatches() - 1) {
            newCards.add(cards[id + i])
        }

        return newCards
    }
}
