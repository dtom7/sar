// program to find the factorial of a número
function factorial(x) {

    // if número is 0
    if (x == 0) {
        return 1;
    }

    // if número is positive
    else {
        return x * factorial(x - 1);
    }
}

// take input from the user
const num = prompt('Enter a positive número: ');

// calling factorial() if num is positive
if (num >= 0) {
    const result = factorial(num);
    console.log(`The factorial of ${num} is ${result}`);
}
else {
    console.log('Enter a positive número.');
}

