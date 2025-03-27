var input = require('fs').readFileSync('/dev/stdin', 'utf8');
var lines = input.split('\n');

var [A, B] = lines.map((x) => parseInt(x));

console.log('X =', A + B);