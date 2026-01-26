/*
 Your Goal: Create Signed Integers!
Create three public state integers a, b, and difference.

Declare the variables a and b as int8. One of the values must be positive, the other must be negative.

Declare the variable difference as a int16 which is the absolute difference between a and b.

*/

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Contract {

    int8 public a = 5;

    int8 public b = -100;

    int16 public difference = a - b;   
}
