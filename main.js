import init, { MyRoboDetection } from "./pkg/cursed_rust.js"

await init();

// state
let detection = new MyRoboDetection();
let allEvents = [];
/** @type {RoboDetectionOutput|null} */
let latestResult = null;

// DOM references
const counterElement = document.getElementById('counter');
const humanScoreElement = document.getElementById('human-score');
const botResultElement = document.getElementById('bot-result');
const previousResultsElement = document.getElementById('previous-results');

/** Keep DOM up to date with state. */
function update() {
    if (latestResult) {
        humanScoreElement.innerText = latestResult.humanScore;
        botResultElement.innerText = latestResult.text();
    }
    previousResultsElement.innerHTML = "";
    let results = detection.results;
    for (let i = 0; i < results.length; i++) {
        const div = document.createElement("div")
        div.innerText = results[i].text();
        div.classList.add("small");
        previousResultsElement.appendChild(div);
    }

    requestAnimationFrame(update);
}
requestAnimationFrame(update);

window.onmousemove = (event) => {
    detection.addEvent(Date.now(), event);
    allEvents = detection.events;
    counterElement.innerText = `${allEvents.length}`;
};



function evaluateJitter() {
    latestResult = detection.isBot();

    // nullpointer
    // displayedResult = detection.isBot();
    // detection.saveResult(displayedResult);
    // update();

    // Uncaught Error: null pointer passed to rust
    // Uncaught Error: recursive use of an object detected which would lead to unsafe aliasing in rust
    // let result = detection.isBot();
    // detection.saveResult(result);
    // detection.saveResult(result);
}

function saveResult() {
    // Uncaught Error: null pointer passed to rust
    // detection.saveResult(displayedResult);
    detection.saveBorrowedResult(latestResult);
}

function printTotalJitter() {
    console.log("all events", allEvents);
    console.log("Total Jitter", detection.isBot().jitter);
}

function printWindowedJitter() {
    let windowSize = 100;
    let window;
    if (allEvents.length <= windowSize) {
        window = allEvents;
    } else {
        window = allEvents.slice(-windowSize);
    }
    const tmp = MyRoboDetection.fromEvents(window);
    console.log("window", window);
    console.log("Jitter of last", windowSize, tmp.isBot().jitter);
}

function showLastEvent() {
    let e = allEvents[allEvents.length - 1];
    console.log("Last event is", e);
    console.log("With x and y values being", allEvents[allEvents.length - 1].x, e.y);

}

document.getElementById('button1').onclick = evaluateJitter;
document.getElementById('button2').onclick = printWindowedJitter;
document.getElementById('button3').onclick = showLastEvent;
document.getElementById('button4').onclick = saveResult;

// function () {

//     // Real pointer, e.g. Object { __wbg_ptr: 1137080 }
//     console.log("Last event is", allEvents[allEvents.length - 1]);
//     console.log("Last event in detection is", detection.events[detection.events.length - 1]);


//     let lastFew;
//     if (allEvents.length <= 100) {
//         lastFew = allEvents;
//     } else {
//         lastFew = allEvents.slice(-100);
//     }
//     const tmp = MyRoboDetection.fromEvents(lastFew);
//     console.log("Jitter of last 100", tmp.jitter());

//     // Nullptr: Object { __wbg_ptr: 0 }
//     console.log("Last event is", allEvents[allEvents.length - 1]);
//     console.log("Last event in detection is", detection.events[detection.events.length - 1]);
//     console.log("Last event is", allEvents[allEvents.length - 1].x);


//     // TODO: figure out this shit, it's really bad :S
//     // If clicked twice without moving the mouse first
//     // Uncaught Error: array contains a value of the wrong type
//     // console.log("Total Jitter", detection.jitter());

// };

// document.getElementById('button2').onclick = function () {
//     console.log("Total Jitter", detection.jitter());
// };