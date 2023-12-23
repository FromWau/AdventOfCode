package day2

import java.io.File

fun main() {

    val allowedRedCubes = 12
    val allowedGreenCubes = 13
    val allowedBlueCubes = 14

    val games = File("input.txt").readLines().map { it.parseLine() }

    // Question 1
    println("Allowed red cubes per draw: $allowedRedCubes")
    println("Allowed green cubes per draw: $allowedGreenCubes")
    println("Allowed blue cubes per draw: $allowedBlueCubes")

    val possibleGames = games.filter { it.isPossible }
    possibleGames.map { it.id }.sum().let { println("Sum of Ids of possible games: $it") }

    // Question 2
    games.map { it.calcPower() }.sum().let { println("Sum of power of games: $it") }
}

fun String.parseLine(): Game {
    var minNeededRedCubes = 0
    var minNeededBlueCubes = 0
    var minNeededGreenCubes = 0
    var roundPossible: List<Boolean> = listOf()
    var drawsList: List<Draw> = listOf()

    val id = this.split(":")[0].replace("Game ", "").toInt()
    val draws = this.split(":")[1].split(";")

    draws.forEach { it ->
        val draw = it.trim()

        var redCubes = 0
        var blueCubes = 0
        var greenCubes = 0

        draw.split(",").forEach {
            val actionPerCube = it.trim()
            val amount = actionPerCube.split(" ")[0].trim().toInt()
            val color = actionPerCube.split(" ")[1].trim()

            when (color) {
                "red" -> redCubes += amount
                "blue" -> blueCubes += amount
                "green" -> greenCubes += amount
            }
        }

        val drawRound = Draw(redCubes = redCubes, blueCubes = blueCubes, greenCubes = greenCubes)
        drawsList += drawRound

        if (redCubes > minNeededRedCubes) minNeededRedCubes = redCubes
        if (blueCubes > minNeededBlueCubes) minNeededBlueCubes = blueCubes
        if (greenCubes > minNeededGreenCubes) minNeededGreenCubes = greenCubes
        roundPossible += drawRound.isPossible()
    }

    val game =
            Game(
                    id = id,
                    isPossible = roundPossible.all { it == true },
                    minNeededRedCubes = minNeededRedCubes,
                    minNeededBlueCubes = minNeededBlueCubes,
                    minNeededGreenCubes = minNeededGreenCubes,
            )
    return game
}

data class Draw(
        val redCubes: Int,
        val blueCubes: Int,
        val greenCubes: Int,
) {
    fun isPossible(): Boolean {
        val allowedRedCubes = 12
        val allowedGreenCubes = 13
        val allowedBlueCubes = 14

        if (redCubes > allowedRedCubes) return false
        if (greenCubes > allowedGreenCubes) return false
        if (blueCubes > allowedBlueCubes) return false

        return true
    }
}

data class Game(
        val id: Int,
        val isPossible: Boolean,
        val minNeededRedCubes: Int,
        val minNeededBlueCubes: Int,
        val minNeededGreenCubes: Int,
) {
    fun calcPower() = minNeededRedCubes * minNeededBlueCubes * minNeededGreenCubes
}
