var input = require('fs').readFileSync('/dev/stdin', 'utf8');
var lines = input.trim().split('\n');

lines.pop();

let T = 0;
let p = 0;

while (p < lines.length) {
    const N = parseInt(lines[p++]);

    if(T) {
        console.log('');
    }

    let consumos = Array(201);
    consumos.fill(0);
    let [totalX, totalY] = [0, 0];
    for(let i = 0; i < N; i++){
        let [X, Y] = lines[p++].split(' ').map((x) => parseInt(x));

        totalX += X;
        totalY += Y;
        consumos[parseInt(Math.floor(Y/X))] += X;
    }

    console.log(`Cidade# ${++T}:`);

    const output = consumos.reduce((prev, curr, i) => {
        if(curr > 0) {
            prev.push(`${curr}-${i}`);
        }
        return prev
    }, []).join(' ');
    console.log(output);

    const consumo_medio = Math.floor((100 * totalY) / totalX) / 100

    console.log(`Consumo medio: ${consumo_medio.toFixed(2)} m3.`);
}