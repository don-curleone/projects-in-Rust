# Wordle
Play wordle in the terminal.
<br>
Made in Rust.

# How the game works
<ul>
  <li>the player gets six chances to guess a five lettered word.</li>
  <li>an array of "_" is used to emulate the yellow and green boxes in the original game</li>
  <li>if a letter is guessed correctly and in the right position, it is indicated by replacing "_" with the letter in the array</li>
  <li>if a letter is guessed correctly but in the wrong position, it is indicated by a "*"</li>
  <li>if a letter is guessed but not in the correct word, the array remains unchanged</li>
</ul>
