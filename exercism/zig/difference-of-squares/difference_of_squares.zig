pub fn squareOfSum(number: usize) usize {
    var result: usize = 0;
    var remaining = number;

    while (remaining != 0) : (remaining -= 1) {
        result += remaining;
    }

    return result * result;
}

pub fn sumOfSquares(number: usize) usize {
    var result: usize = 0;

    for (0..number + 1) |n| {
        result += n * n;
    }

    return result;
}

pub fn differenceOfSquares(number: usize) usize {
    return squareOfSum(number) - sumOfSquares(number);
}
