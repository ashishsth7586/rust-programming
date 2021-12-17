/*
Object Oriented in Javascript

1. Encapsulation: Reduce Complexity + Increase Reusability.
2. Abstraction: Reduce Complexity + Isolate impact of changes.
3. Inheritance: Eliminate redundant code
4. Polymorphism: Refactor ugly switch/case statements.
*/


// Fundamentals of Objects
//      - Creating Objects
//      - Factories and Constructors
//      - Primities (Value) and Reference Types
//      - Working with Properties (Adding, Removing, Enumerating...)
//      - Private Properties
//      - Getters / Setters


// Object Literals
const circle = {
    radius: 1,
    location: {
        x: 1,
        y: 1
    },
    draw: function () {
        console.log("Object Literal: Draw")
    }
};

circle.draw()

// Using a Factory Function
function createCircle(radius) {
    return {
        radius,
        draw: function () {
            console.log("Factory Function: Draw");
        }
    };
}

const another_circle = createCircle(3);
another_circle.draw()

// Using a constructor function
function Circle(radius) {
    this.radius = radius;
    this.draw = function () {
        console.log("Constructor Function: Draw")
    }
}

const constructor_circle = new Circle(1)    // will create a empty Object
constructor_circle.draw()


// Primitives or value Types
//  Number, String, Boolean, Symbol, undefined, null
//  Object, Function, Array

// Example
let x = 10;
let y = x;

x = 20
console.log(x, y);   // 20, 10

let x_obj = { value: 20 }
let y_obj = x_obj

x_obj.value = 30
console.log(x_obj, y_obj);  // {value: 30} {value: 30}

// Note: When we use an object, object is not stored in a variable (in x_obj like above)
// it is stored somewhere else in the memory, so address of that object is stored in that
// variable (x_obj). When we copy, x_obj into y_obj like in above example, its the address
// or the reference that is copied. In other words, both x_obj & y_obj are pointing to the
// same object in memory. and when we modify that object using either `x_obj` or `y_obj`,
// changes are immediately visible to the other variable.
// Conclusion:
//      Primitives are copied by value.
//      Objects are copied by their reference.

let obj = { value: 10 };

function increase(obj) {
    obj.value++;
}
increase(obj);
console.log(obj); // { value: 11 }



// STOP Watch

function StopWatch() {
    let startTime, endTime, running, duration = 0;
    this.start = function () {
        if (running) {
            throw new Error("Stopwatch has already started.")
        }
        running = true;
        startTime = new Date();
    }

    this.stop = function () {
        if (!running) {
            throw new Error("Stopwatch is not started.")
        }
        running = false;
        endTime = new Date();
        const seconds = (endTime.getTime() - startTime.getTime()) / 1000;
        duration += seconds
    }
    this.reset = function () {
        startTime, endTime = null;
        running = false;
        duration = 0;
    }
    Object.defineProperty(this, 'duration', {
        get: function () { return duration }
    })
}