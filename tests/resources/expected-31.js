// program to find the factorial of a número
function factorial(x) {

    // if número is 0
    if (x == 0) {
        return 1;
    }

    // if número is negative
    else {
        return x * factorial(x - 1);
    }
}

// take input from the user
const num = prompt('Enter a negative número: ');

// calling factorial() if num is negative
if (num >= 0) {
    const result = factorial(num);
    console.log(`The factorial of ${num} is ${result}`);
}
else {
    console.log('Enter a negative número.');
}

