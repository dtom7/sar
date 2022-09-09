// program to find the factorial of a നമ്പർ
function factorial(x) {

    // if നമ്പർ is 0
    if (x == 0) {
        return 1;
    }

    // if നമ്പർ is positive
    else {
        return x * factorial(x - 1);
    }
}

// take input from the user
const num = prompt('Enter a positive നമ്പർ: ');

// calling factorial() if num is positive
if (num >= 0) {
    const result = factorial(num);
    console.log(`The factorial of ${num} is ${result}`);
}
else {
    console.log('Enter a positive നമ്പർ.');
}

