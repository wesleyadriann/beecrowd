var input = require("fs").readFileSync("/dev/stdin", "utf8");
var lines = input.trim();

var nomes = [
  "Dasher",
  "Dancer",
  "Prancer",
  "Vixen",
  "Comet",
  "Cupid",
  "Donner",
  "Blitzen",
  "Rudolph",
];

var total = 0;

lines.split(" ").forEach((curr) => {
  total += parseInt(curr);
});

var result = total % 9;

if (result === 0) result = 9;

console.log(nomes[result - 1]);
