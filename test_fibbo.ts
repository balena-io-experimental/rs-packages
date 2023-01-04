let rs_imp = require('.');

console.log("Testing Rust:")
let start = Date.now();
rs_imp.JsToRustStreamString.fibonacci(40)
console.log("Duration", Date.now() - start);

function fibonacci(n: number) : number {
    if (n == 0 || n == 1) {
        return 1;
    }else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

console.log("Testing JS:")
start = Date.now();
fibonacci(40)
console.log("Duration", Date.now() - start);